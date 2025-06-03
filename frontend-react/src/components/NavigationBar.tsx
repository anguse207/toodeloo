import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import MenuIcon from '@mui/icons-material/Menu';
import React from "react";
import Drawer from "@mui/material/Drawer";
import { ListItem, ListItemButton, ListItemIcon, ListItemText, Divider } from "@mui/material";
import ChecklistRtlIcon from '@mui/icons-material/ChecklistRtl';
import AddCircleOutlineIcon from '@mui/icons-material/AddCircleOutline';
import AddTaskIcon from '@mui/icons-material/AddTask';
import { CreateList } from "../api/CreateList";
import { useNavigate } from "react-router-dom";

// Define the type for a list item
export interface ListItem {
  id: string;
  title: string;
}

// Props for the ListSelector component
export interface ListSelectorProps {
  lists: ListItem[] | null; // Null indicates loading state
}

const ListSelector: React.FC<ListSelectorProps> = ({ lists }) => {
  const [open, setOpen] = React.useState(false);
  const navigate = useNavigate();


  const toggleDrawer = (newOpen: boolean) => () => {
    setOpen(newOpen);
  };

  const createList = async () => {
    const list_id = await CreateList();

    // navigate to list_id
    if (list_id) {
      
      navigate(`/${list_id}`);
    }
  }

  const DrawerList = (
      <Box sx={{ width: 250 }} role="presentation" onClick={toggleDrawer(false)}>
      <ListItem key="AddTask" disablePadding sx={{ marginTop: 2 }}>
          <ListItemButton>
              <ListItemText primary="Task" />
              <ListItemIcon>
                  <AddTaskIcon/>
              </ListItemIcon>
          </ListItemButton>
      </ListItem>
      <Divider />
      <ListItem key="AddList" disablePadding sx={{ marginTop: 2 }}>
          <ListItemButton onClick={createList}>
              <ListItemText primary="List" />
              <ListItemIcon>
                  <AddCircleOutlineIcon/>
              </ListItemIcon>
          </ListItemButton>
      </ListItem>
      <Divider />
      {lists?.map((list) => (
        <ListItem key={list.id} disablePadding sx={{ marginTop: 1 }}>
          <ListItemButton>
            <ListItemText primary={list.title} />
            <ListItemIcon>
              <ChecklistRtlIcon/>
            </ListItemIcon>
          </ListItemButton>
        </ListItem>
      ))}
      </Box>
    );

  return (
    <>
      <AppBar position="fixed" sx={{ width: '100%' }}>
        <Toolbar>
          <IconButton
            size="large"
            edge="start"
            color="inherit"
            aria-label="menu"
            sx={{ mr: 2 }}
            onClick={toggleDrawer(true)}
          >
            <MenuIcon />
            </IconButton>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            / TDLO /
          </Typography>
          <Button onClick={toggleDrawer(true)} color="inherit">
              Login
          </Button>
        </Toolbar>
      </AppBar>
      {/* Add padding to the content below the AppBar to prevent overlap */}
      <Box sx={{ height: '64px' }} />
      <Drawer open={open} onClose={toggleDrawer(false)}>
          {DrawerList}
      </Drawer>
    </>
  );
}

export default ListSelector;