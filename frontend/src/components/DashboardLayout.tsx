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
import ListTitle from "./ListTitle";
import { ReadLists } from "../api/ReadLists";

// Define the type for a list item
export interface ListItem {
  id: string;
  title: string;
}

// Props for the ListSelector component
export interface DashboardLayoutProps {
  selectedListId: string | undefined;
}


const drawerWidth = 500;

const DashboardLayout: React.FC<DashboardLayoutProps> = ({ selectedListId }) => {
  const [mobileOpen, setMobileOpen] = React.useState(false);
  const [isClosing, setIsClosing] = React.useState(false);
  const [listTitleIsDirty, setListTitleIsDirty] = React.useState(false);
  const [listSelectorItems, setListSelectorItems] = React.useState<ListItem[] | null>(null);

  useEffect(() => {
    refreshListSelectorItems();
  }, [selectedListId, listTitleIsDirty]);

  const refreshListSelectorItems = async () => {
    const lists_to_set: ListItem[] = [];
    const newLists = await ReadLists() ?? [];

    if (newLists) {
      for (const list of newLists) {
        lists_to_set.push({ id: list.id, title: list.label });
        lists_to_set.reverse();
      }
    }

    setListSelectorItems(lists_to_set);
  }

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
    const list_id = await CreateList("");

    // navigate to list_id
    if (list_id) {
      navigate(`/${list_id}`, { replace: true });
    }
  }

  const switchList = async (listId: string) => {
    // navigate to list_id
    while (listTitleIsDirty) {
      console.warn("Waiting for changes to be saved...");
      await new Promise((resolve) => {
        const interval = setInterval(() => {
          if (!listTitleIsDirty) {
            clearInterval(interval);
            resolve(null);
          }
        }, 100); // Check every x ms
      });
    }

    navigate(`/${listId}`, { replace: true });
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
        {listSelectorItems?.map((list) => (
          <ListItem key={list.id} disablePadding sx={{ marginTop: 1 }}>
            <ListItemButton onClick={() => switchList(`${list.id}`)} >
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

      <Box sx={{ marginLeft: drawerWidth / 10 }}>
        {selectedListId && <ListTitle listId={selectedListId} setIsDirty={setListTitleIsDirty}/>}
      </Box>

    </>
  );
}

export default DashboardLayout;