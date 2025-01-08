#
# Description: This is the main entry point for the Ava AI API
# Author: Alexander Powell
# Version: v1.0
# Dependencies: Flask, jsonify
#

from flask import Flask, jsonify, request
from dotenv import load_dotenv, find_dotenv
from ava_ai import AvaAI

import os


api: Flask = Flask(__name__)
ava: AvaAI = AvaAI()

load_dotenv(find_dotenv(), override=True)


@api.route("/load_doc", methods=["POST"])
def load_document():
    """
    Tell the AI where to find the document so 
    the AI can chunk and embed it.

    Returns:
        jsonify json data and status code
    """
    try:
        data = request.json

        if data["message"] == "File copied successfully":
                file = data["new_name"]
                file_location = os.environ["DOCUMENT_PATH"]
                doc = file_location + "/" + file

                ava.load_doc(doc)
                ava.chunk_data()
                ava.delete_pinecone_index()
                ava.insert_or_fetch_ebeddings()
            
        return jsonify({"message": "Data recieved!"}), 200
    
    except Exception as e:
        print(e)
        return jsonify({"error": str(e)}), 400


@api.route("/get_ai_response", methods=["POST"])
def get_ai_response():
    """
    Get human question and send the AI response back.

    Returns:
        jsonify json data and status code
    """
    
    try:
        data = request.json

        q = data["message"]
        answer = ava.ask_and_get_answer(q)

        return jsonify({"avas_response": answer["result"]}), 200
        
    except Exception as e:
        print(e)
        return jsonify({"error": str(e)}), 400


if __name__ == "__main__":
    api.run(debug=True, port=5001)
