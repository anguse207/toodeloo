import { useParams } from 'react-router-dom';
import DashboardLayout from './components/DashboardLayout';

const Home: React.FC = () => {
  const { selectedListId } = useParams<{ selectedListId: string | undefined }>();

  return (
    <>
      <DashboardLayout selectedListId={selectedListId}/>
    </>
  );
};

export default Home;