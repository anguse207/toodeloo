import { HOST } from "./Constants";

// Define the TypeScript interface matching the Rust struct
export interface List {
  id: string;
  user_id: string;
  label: string;
  deleted_time: number; // Assuming i64 maps to number in TypeScript
}

export const ReadLists = async (): Promise<List[] | undefined> => {
  const url = HOST + "/api/lists";

  try {
    const response = await fetch(url, {
      method: 'GET',
      credentials: 'include',
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json(); // Ensure the response matches the List interface
    console.log('Response data:', data);

    return data; // Return the List object
  } catch (error) {
    // console.error('Error posting data:', error);
    console.log('Response data:', error);

    return undefined;
    // window.location.href = '/login';
    // throw error; // Re-throw the error for the caller to handle
  }
};