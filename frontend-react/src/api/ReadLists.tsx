// Define the TypeScript interface matching the Rust struct
export interface List {
  id: string;
  user_id: string;
  label: string;
  deleted_time: number; // Assuming i64 maps to number in TypeScript
}

export const ReadLists = async (): Promise<List[] | undefined> => {
  const url = '/api/lists/create'; // Replace with your API endpoint
  const body = {
    label: 'My New List',
  };

  try {
    const response = await fetch(url, {
      method: 'POST',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data: List[] = await response.json(); // Ensure the response matches the List interface
    console.log('Response data:', data);
    return data; // Return the List object
  } catch (error) {
    console.error('Error posting data:', error);
    return undefined;
    // window.location.href = '/login';
    // throw error; // Re-throw the error for the caller to handle
  }
};