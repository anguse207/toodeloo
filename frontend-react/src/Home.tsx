import React, { useState, useEffect } from 'react';
import ListSelector from './components/ListSelector';
import Button from '@mui/material/Button';

interface ListItem {
  id: string;
  title: string;
}

const Home: React.FC = () => {
  const [lists, setLists] = useState<ListItem[] | null>(null);

  useEffect(() => {
    // Simulate loading lists from an API
    setTimeout(() => {
      setLists([
        { id: '1', title: 'Groceries' },
        { id: '2', title: 'Work' },
        { id: '3', title: 'Personal' },
      ]);
    }, 1000);

    setTimeout(() => {
      setLists([
        { id: '4', title: 'Hobbies' },
        { id: '5', title: 'Travel' },
        { id: '6', title: 'Fitness' },
        { id: '7', title: 'Books' },
        { id: '8', title: 'Movies' },
        { id: '9', title: 'Music' },
        { id: '10', title: 'Games' },
        { id: '11', title: 'Projects' },
        { id: '12', title: 'Shopping' },
        { id: '13', title: 'Events' },
        { id: '14', title: 'Ideas' },]);
    }, 4000);
  }, []);

  const handlePostRequest = async () => {
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

      const data = await response.json();
      console.log('Response data:', data);
    } catch (error) {
      console.error('Error posting data:', error);
      // redirect to login page
      window.location.href = '/login';
      
    }
  };

  const handleListChange = (selectedListId: string | null) => {
    console.log('Selected List ID:', selectedListId);
  };

  return (
    <div>
      <Button onClick={() => handlePostRequest()} variant='contained'>Click me!</Button>
      <ListSelector lists={lists} onListChange={handleListChange} />
    </div>
  );
};

export default Home;