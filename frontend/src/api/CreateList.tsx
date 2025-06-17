import { HOST } from "./Constants";

export const CreateList = async (listLabel: string): Promise<string | undefined> => {
  const url = HOST + "/api/lists/create";

  const body = {
    label: listLabel,
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

    const id: string = await response.json(); // Expect the server to return the ID of the created list
    console.log("Created list: ", id);

    return id;
  } catch (error) {
    console.error('Error posting data:', error);
    return undefined;
  }
};