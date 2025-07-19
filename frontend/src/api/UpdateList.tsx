import { HOST } from "./Constants";

export const UpdateList = async (listId: string, listLabel: string) => {
  const url = HOST + "/api/lists/update/" + listId;

  const body = {
    label: listLabel,
  };

  try {
    const response = await fetch(url, {
      method: 'PUT',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    console.log("Updated list: ", listId);
  } catch (error) {
    console.error('Error posting data:', error);
    return undefined;
  }
};