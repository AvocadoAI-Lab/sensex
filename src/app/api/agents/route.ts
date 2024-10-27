import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { endpoint, token } = await request.json();

    // Ensure we have the required fields
    if (!endpoint || !token) {
      return NextResponse.json(
        { error: 'Missing required fields: endpoint and token' },
        { status: 400 }
      );
    }

    // Forward the request to Rust backend with the correct structure
    const response = await fetch('http://localhost:3001/agents', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        endpoint,
        token,
      }),
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
