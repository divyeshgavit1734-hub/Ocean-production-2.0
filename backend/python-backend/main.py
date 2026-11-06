from fastapi import FastAPI, HTTPException
from database import get_supabase_client

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


@app.get("/db-health")
def database_health():
    try:
        supabase = get_supabase_client()

        result = (
            supabase.table("customers")
            .select("customer_id")
            .limit(1)
            .execute()
        )

        return {
            "status": "connected",
            "database": "Supabase",
            "message": "Ocean Production database connection successful"
        }

    except Exception:
        raise HTTPException(
            status_code=503,
            detail="Database connection or permission check failed"
        )