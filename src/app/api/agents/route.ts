import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    // Forward the request to Rust backend
    const response = await fetch('http://localhost:3001/agents', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        // Forward the authorization header if present
        ...request.headers
      },
      body: JSON.stringify(await request.json()),
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
