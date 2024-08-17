from groq import Groq
import os
import sys
from dotenv import load_dotenv

def main():
    if len(sys.argv) != 2:
        print("Usage: python3 groq_api.py '<prompt>'")
        sys.exit(1)

    prompt = sys.argv[1]

    # Load the .env file
    load_dotenv()

    # Access the API key from the environment variable
    api_key = os.getenv("GROQ_API_KEY")
    if not api_key:
        print("Error: GROQ_API_KEY environment variable not set.")
        sys.exit(1)

    # Initialize Groq client with the API key
    client = Groq(api_key=api_key)

    # Create completion using the Groq client
    try:
        completion = client.chat.completions.create(
            model="llama3-8b-8192",
            messages=[
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            temperature=1.0,
            max_tokens=1024,
            top_p=1.0,
            stream=False,
            stop=None,
        )

        # Output the generated story
        for chunk in completion:
            # Print the type and structure of chunk for debugging
            print(f"Received chunk: {chunk}")
            
            # Ensure the chunk is what you expect
            if isinstance(chunk, dict) and 'choices' in chunk:
                for choice in chunk['choices']:
                    if 'delta' in choice and 'content' in choice['delta']:
                        print(choice['delta']['content'] or "", end="")
            else:
                print("Unexpected chunk format")

    except Exception as e:
        print(f"Error occurred: {e}")

if __name__ == "__main__":
    main()
