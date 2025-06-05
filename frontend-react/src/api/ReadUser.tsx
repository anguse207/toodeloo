import { HOST } from "./Constants";

// Define the TypeScript interface matching the Rust struct
export interface User {
  id: string,
  oauth_id: string,
  oauth_provider: string,
  nickname: string,
  deleted_time: number,
}

export const ReadUser = async (): Promise<User | undefined> => {
  const url = HOST + "/api/users";

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