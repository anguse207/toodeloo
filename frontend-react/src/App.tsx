// src/App.js
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';

import Login from './Login';
import Home from './Home';
import Tasks from './Tasks';

const App = () => {
    return (
        <Router>
            <Routes>
                <Route path="/login" element={<Login />} />
                <Route path="/tasks" element={<Tasks />} />
                <Route path="/" element={<Home />} />
            </Routes>
        </Router>
    );
};

export default App;
