import argparse
from langchain_community.vectorstores import Chroma
from langchain_openai import OpenAIEmbeddings
from langchain_openai import ChatOpenAI
from langchain.prompts import ChatPromptTemplate

from fastapi.middleware.cors import CORSMiddleware
from fastapi import FastAPI

app = FastAPI()

origins = [
    "*",
]

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

CHROMA_PATH = "chroma"

PROMPT_TEMPLATE = """
You are an AI that answers questions about the Rust programming language. Your job is to answer the questions honestly and accurately based on the provided context.

{context}

---

Answer the question based on the above context: {question}
"""

from pydantic import BaseModel

class Question(BaseModel):
    text: str

@app.post("/ask")
def get_res(question: Question):
    query_text = question.text
    print(query_text)

    # Prepare the DB.
    embedding_function = OpenAIEmbeddings()
    db = Chroma(persist_directory=CHROMA_PATH, embedding_function=embedding_function)

    # Search the DB.
    results = db.similarity_search_with_relevance_scores(query_text, k=3)
    if len(results) == 0:
        print(f"Unable to find matching results.")
        return {"res": "Unable to find matching results."}

    context_text = "\\n\\n---\\n\\n".join([doc.page_content for doc, _score in results])
    prompt_template = ChatPromptTemplate.from_template(PROMPT_TEMPLATE)
    prompt = prompt_template.format(context=context_text, question=query_text)

    model = ChatOpenAI()
    response_text = model.predict(prompt)

    sources = [doc.metadata.get("source", None) for doc, _score in results]
    formatted_response = f"Response: {response_text}\\nSources: {sources}"
    print(formatted_response)
    return {"res": response_text}




"""

curl -X POST -H "Content-Type: application/json" -d '{"text": "
I have a rust function 

fn sum(vec) -> i32 {
    for i in range(vec.len){
        sum += vec[i]
    }
    sum
}

whats wrong with it
"}' http://localhost:8000/ask

"""

