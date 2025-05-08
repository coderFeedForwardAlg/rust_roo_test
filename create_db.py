from langchain_community.document_loaders import PyPDFLoader
from langchain.text_splitter import RecursiveCharacterTextSplitter
from langchain.schema import Document
from langchain_openai import OpenAIEmbeddings
from langchain_community.vectorstores import Chroma
import openai
from dotenv import load_dotenv
import os
import shutil

load_dotenv()

openai.api_key = os.environ['OPENAI_API_KEY']

CHROMA_PATH = 'chroma'

DATA_PATH = '.'  # Changed to current directory

def main():
    generate_data_store()

def generate_data_store():
    docs = load_documents()
    chunks = split_text(docs)
    save_to_chroma(chunks)

def load_documents():
    loader = PyPDFLoader("rust.pdf")  # Load the PDF file
    docs = loader.load()
    return docs

def split_text(docs: list[Document]):
    text_splitter = RecursiveCharacterTextSplitter(
        chunk_size = 300,
        chunk_overlap = 100,
        length_function = len,
        add_start_index = True
    )
    chunks = text_splitter.split_documents(docs)
    return chunks

def save_to_chroma(chunks: list[Document]):
    if os.path.exists(CHROMA_PATH):
        shutil.rmtree(CHROMA_PATH)

    # Create a new DB from the documents.
    db = Chroma.from_documents(
        chunks, OpenAIEmbeddings(), persist_directory=CHROMA_PATH
    )
    db.persist()
    print(f"Saved {len(chunks)} chunks to {CHROMA_PATH}.")

if __name__ == "__main__":
    main()