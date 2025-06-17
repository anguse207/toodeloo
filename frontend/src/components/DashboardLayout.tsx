import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import MenuIcon from '@mui/icons-material/Menu';
import React, { useEffect } from "react";
import Drawer from "@mui/material/Drawer";
import { ListItem, ListItemButton, ListItemIcon, ListItemText, Divider } from "@mui/material";
import ChecklistRtlIcon from '@mui/icons-material/ChecklistRtl';
import AddCircleOutlineIcon from '@mui/icons-material/AddCircleOutline';
import AddTaskIcon from '@mui/icons-material/AddTask';
import { CreateList } from "../api/CreateList";
import { useNavigate } from "react-router-dom";
import { ReadUser } from "../api/ReadUser";
import LoginDialog from "./LoginDialog";

// Define the type for a list item
export interface ListItem {
  id: string;
  title: string;
}

// Props for the ListSelector component
export interface ListSelectorProps {
  lists: ListItem[] | null; // Null indicates loading state
}


const drawerWidth = 300;

const DashboardLayout: React.FC<ListSelectorProps> = ({ lists }) => {
  const [mobileOpen, setMobileOpen] = React.useState(false);
  const [isClosing, setIsClosing] = React.useState(false);

  const handleDrawerClose = () => {
    setIsClosing(true);
    setMobileOpen(false);
  };

  const handleDrawerTransitionEnd = () => {
    setIsClosing(false);
  };

  const handleDrawerToggle = () => {
    if (!isClosing) {
      setMobileOpen(!mobileOpen);
    }
  };
  
  const [nickname, setNickname] = React.useState("");
  const [LoginDialogOpen, setLoginDialogOpen] = React.useState(false);
  const navigate = useNavigate();

    useEffect(() => {
      fetchUser();
    }, []);

  const fetchUser = async () => {
    const user = await ReadUser();

    if (user) {
      setNickname(user!.nickname + " - " + user!.oauth_provider);
    } else {
      setLoginDialogOpen(true);
    }
  };

  const createList = async () => {
    const list_id = await CreateList("super cali fragilistic expialidocious, super cali fragilistic expialidocious ");

    // navigate to list_id
    if (list_id) {
      navigate(`/${list_id}`, { replace: true });
    }
  }

  const logout = async () => {
    // Delete the auth-token cookie
    // document.cookie = "auth-token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
    // Needs to be deleted server side
    setLoginDialogOpen(true);
  };

  const DrawerList = (
      <Box sx={{ width: '100%' }} role="presentation" >
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
            <ListItemButton onClick={() => navigate(`/${list.id}`, { replace: true })} >
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
      {/* Login popup */}
      <LoginDialog open={LoginDialogOpen}/>

      <Box sx={{ display: 'flex' }}>
        <AppBar
          position="fixed"
          sx={{
            width: { sm: `calc(100% - ${drawerWidth}px)` },
            ml: { sm: `${drawerWidth}px` },
          }}
        >
          <Toolbar>
            <IconButton
              color="inherit"
              aria-label="open drawer"
              edge="start"
              onClick={handleDrawerToggle}
              sx={{ mr: 2, display: { sm: 'none' } }}
            >
              <MenuIcon />
            </IconButton>
            <Typography variant="h6" noWrap component="div">
              toodeloo
            </Typography>
            <Button onClick={logout} color="inherit" sx={{ marginLeft: 'auto' }}>
              {nickname}
            </Button>
          </Toolbar>
        </AppBar>

        <Box
          component="nav"
          sx={{ width: { sm: drawerWidth }, flexShrink: { sm: 0 } }}
          aria-label="mailbox folders"
        >
          {/* The implementation can be swapped with js to avoid SEO duplication of links. */}
          <Drawer
            variant="temporary"
            open={mobileOpen}
            onTransitionEnd={handleDrawerTransitionEnd}
            onClose={handleDrawerClose}
            sx={{
              display: { xs: 'block', sm: 'none' },
              '& .MuiDrawer-paper': { boxSizing: 'border-box', width: drawerWidth },
            }}
            slotProps={{
              root: {
                keepMounted: true, // Better open performance on mobile.
              },
            }}
          >
            {DrawerList}
          </Drawer>
          <Drawer
            variant="permanent"
            sx={{
              display: { xs: 'none', sm: 'block' },
              '& .MuiDrawer-paper': { boxSizing: 'border-box', width: drawerWidth },
            }}
            open
          >
            {DrawerList}
          </Drawer>
        </Box>
      </Box>
    </>
  );
}

export default DashboardLayout;