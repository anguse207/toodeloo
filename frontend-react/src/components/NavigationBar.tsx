import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import MenuIcon from '@mui/icons-material/Menu';
import React from "react";
import Drawer from "@mui/material/Drawer";
import { List, ListItem, ListItemButton, ListItemIcon, ListItemText, Divider } from "@mui/material";
import ChecklistRtlIcon from '@mui/icons-material/ChecklistRtl';
import AddCircleOutlineIcon from '@mui/icons-material/AddCircleOutline';
import AddTaskIcon from '@mui/icons-material/AddTask';

export default function ButtonAppBar() {
    const [open, setOpen] = React.useState(false);

    const toggleDrawer = (newOpen: boolean) => () => {
      setOpen(newOpen);
    };

    const DrawerList = (
        <Box sx={{ width: 250 }} role="presentation" onClick={toggleDrawer(false)}>
        <ListItem key="AddTask" disablePadding sx={{ marginTop: 2 }}>
            <ListItemButton>
                <ListItemIcon>
                    <AddTaskIcon/>
                </ListItemIcon>
                <ListItemText primary="Task" />
            </ListItemButton>
        </ListItem>
        <Divider />
        <ListItem key="AddList" disablePadding sx={{ marginTop: 2 }}>
            <ListItemButton>
                <ListItemIcon>
                    <AddCircleOutlineIcon/>
                </ListItemIcon>
                <ListItemText primary="List" />
            </ListItemButton>
        </ListItem>
        <Divider />
        <List>
        {['Shoppings', 'Workouts', 'Birthday Presents'].map((text) => (
            <ListItem key={text} disablePadding>
            <ListItemButton>
                <ListItemIcon>
                    <ChecklistRtlIcon/>
                </ListItemIcon>
                <ListItemText primary={text} />
            </ListItemButton>
            </ListItem>
        ))}
        </List>
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