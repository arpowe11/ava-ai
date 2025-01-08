# Ava AI Reloaded Project Requirements

Here are the requirements for the project that each aspect of the project 
needs to be able to do.

<br>

- ### CLI tool
    - **Coding Language:** Rust
        - Interface for the user to interact with
        - Must be able to load the document and send the document name to the API for the AI to chunk and embed
        - Must be able to chat with the AI
        - Must be able to exit the application

<br>

- ### API
    - **Coding Language:** Python
        - Connect to the CLI and the AI to send human questions and AI responses between the two applications
        - Get and load the document

<br>

- ### Ava AI Model
    - **Coding Language:** Python
        - Chunk and embed a document
        - Recieve quesitons about that document and return the AI response