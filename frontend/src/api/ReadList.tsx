import { HOST } from "./Constants";

export interface List {
  id: string;
  user_id: string;
  origin_time: number;
  label: string;
  deleted_time: number;
}

export const ReadList = async (listId: string): Promise<List | undefined> => {
  const url = HOST + "/api/lists/" + listId;

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