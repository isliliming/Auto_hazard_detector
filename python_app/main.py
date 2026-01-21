from fastapi import FastAPI
import web_hazard_project  # Importing the Rust library!

app = FastAPI()

@app.get("/")
def home():
    # Use Rust to do math
    result = web_hazard_project.sum_as_string(10, 20)
    return {"message": "Rust calculated this:", "result": result}
