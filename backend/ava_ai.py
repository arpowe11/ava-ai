# Load the .env file and its variables

import os
import time
from dotenv import load_dotenv, find_dotenv

load_dotenv(find_dotenv(), override=True)

class AvaAI:
    def __init__(self):
        self.data = None
        self.chunks = None
        self.vector_store = None

    def load_doc(self, file):
        import os

        name, extension = os.path.splitext(file)  # get the name and extension of the file

        # only load PDF files and Word Documents
        if extension == ".pdf":
            from langchain_community.document_loaders import PyPDFLoader
            print(f"Loading {file}")
            loader: PyPDFLoader = PyPDFLoader(file)
        elif extension == ".docx":
            from langchain.document_loaders import Docx2txtLoader
            print(f"Loading {file}")
            loader: Docx2txtLoader = Docx2txtLoader(file)
        else:
            print("Document not supported")
            return None

        data = loader.load()
        self.data = data

    def chunk_data(self, chunck_size=256):
        data = self.data

        from langchain.text_splitter import RecursiveCharacterTextSplitter
        text_splitter: RecursiveCharacterTextSplitter = RecursiveCharacterTextSplitter(chunk_size=chunck_size, chunk_overlap=0)
        chunks = text_splitter.split_documents(data)
        self.chunks = chunks

    @staticmethod
    def print_embedding_cost(texts):
        import tiktoken
        enc = tiktoken.encoding_for_model("text-embedding-ada-002")
        total_tokens = sum([len(enc.encode(page.page_content)) for page in texts])
        print(f"Total Tokens: {total_tokens}")
        print(f"Ebedding Cost in USD: {total_tokens / 1000 * 0.0004:.6f}")


    def insert_or_fetch_ebeddings(self):
        import pinecone
        from langchain_community.vectorstores import Pinecone
        from langchain_openai import OpenAIEmbeddings
        from pinecone import PodSpec, ServerlessSpec

        pc = pinecone.Pinecone()  # already loaded in above cells, no need to explicitly pass API key
        embeddings: OpenAIEmbeddings = OpenAIEmbeddings(model="text-embedding-3-small", dimensions=1536)
        index_name = "askadocument"
        chunks = self.chunks

        # Check to see if the index exists before loading, if not create one
        if index_name in pc.list_indexes().names():
            print(f"Index {index_name} already exists. Loading embeddings ... ", end='')
            vector_store = Pinecone.from_existing_index(index_name, embeddings)
            print("Ok")
        else:
            print(f"Creating index {index_name} and embeddings ... ", end='')
            pc.create_index(
                name=index_name,
                dimension=1536,
                metric="cosine",
                spec=ServerlessSpec(cloud="aws", region="us-east-1") #PodSpec(environment="gcp-starter")
            )

            vector_store = Pinecone.from_documents(chunks, embeddings, index_name=index_name)  # for texts try .from_texts()
            print("Ok")
            self.vector_store = vector_store


    # Deleting an index from the vector database
    def delete_pinecone_index(self, index_name="all"):
        import pinecone

        pc = pinecone.Pinecone()
        if index_name == "all":
            indexes = pc.list_indexes().names()
            print("Deleting all indexes ... ")
            for index in indexes:
                pc.delete_index(index)
            print("Ok")
        else:
            print(f"Deleting index {index_name} .... ", end='')
            pc.delete_index(index)
            print("Ok")


    def ask_and_get_answer(self, q):
        from langchain.chains import RetrievalQA
        from langchain_openai import ChatOpenAI
        
        vector_store = self.vector_store

        llm: ChatOpenAI = ChatOpenAI(model_name="gpt-3.5-turbo", temperature=1)
        retriever = vector_store.as_retriever(search_type="similarity", search_kwargs={'k': 3})
        chain = RetrievalQA.from_chain_type(llm=llm, chain_type="stuff", retriever=retriever)

        answer = chain.invoke(q)
        return answer
