import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const credentials = await request.json();

    // Forward the request to our Rust backend
    const response = await fetch('http://localhost:3001/auth', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(credentials),
    });

    const data = await response.json();
    return NextResponse.json(data);
  } catch (error) {
    return NextResponse.json(
      { error: (error as Error).message },
      { status: 500 }
    );
  }
}
