from fastapi import FastAPI

app = FastAPI(
    title="Ocean Production API",
    description="Backend API for Ocean Production",
    version="1.0.0"
)

@app.get("/")
def home():
    return {
        "message": "Ocean Production Python Backend is running!",
        "status": "success"
    }

@app.get("/health")
def health():
    return {
        "status": "healthy"
    }