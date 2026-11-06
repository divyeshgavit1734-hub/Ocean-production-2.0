import os
from supabase import create_client, Client


def get_supabase_client() -> Client:
    supabase_url = os.getenv("https://hsdgxktuutisvrsaieeq.supabase.co")
    supabase_key = os.getenv("sb_publishable_Tw6uv77lblkcb9LHaSlvEQ_5sTCDFD1")

    if not supabase_url or not supabase_key:
        raise RuntimeError(
            "Supabase environment variables are missing"
        )

    return create_client(supabase_url, supabase_key)