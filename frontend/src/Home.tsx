  import React, { useState, useEffect } from 'react';
import { type ListItem } from './components/DashboardLayout';
import { useParams } from 'react-router-dom';
import DashboardLayout from './components/DashboardLayout';
import { ReadLists } from './api/ReadLists';
import ListTextField from './components/ListEditor';

const Home: React.FC = () => {
  const { listId } = useParams<{ listId: string | undefined }>(); // Access the taskId parameter
  const [listSelectorItems, setListSelectorItems] = useState<ListItem[] | null>(null);

  useEffect(() => {
    refreshListSelectorItems();
    console.log(listId);
  }, [listId]);

  const refreshListSelectorItems = async () => {
    const lists_to_set: ListItem[] = [];
    const newLists = await ReadLists() ?? [];

    if (newLists) {
      for (const list of newLists) {
        lists_to_set.push({ id: list.id, title: list.label });
      }
    }

    setListSelectorItems(lists_to_set);
  }

  return (
    <>
      <DashboardLayout lists={listSelectorItems}/>
      <ListTextField listId={listId}/>
    </>
  );
};

export default Home;