import os
from supabase import create_client, Client


def get_supabase_client() -> Client:
    supabase_url = os.getenv("SUPABASE_URL")
    supabase_key = os.getenv("SUPABASE_KEY")

    if not supabase_url or not supabase_key:
        raise RuntimeError(
            "Supabase environment variables are missing"
        )

    return create_client(supabase_url, supabase_key)