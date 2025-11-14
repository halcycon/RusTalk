import { useEffect, useState } from 'react';
import {
  Box, Card, CardContent, Typography, Button, Grid, Alert, CircularProgress,
  Switch, FormControlLabel, Table, TableBody, TableCell, TableContainer, TableHead,
  TableRow, Paper, Chip, Dialog, DialogTitle, DialogContent, DialogActions,
  TextField, IconButton, Tooltip,
} from '@mui/material';
import {
  Add as AddIcon, Delete as DeleteIcon, Refresh as RefreshIcon,
  Edit as EditIcon, DragIndicator as DragIcon,
} from '@mui/icons-material';
import {
  DndContext, closestCenter, KeyboardSensor, PointerSensor,
  useSensor, useSensors,
} from '@dnd-kit/core';
import type { DragEndEvent } from '@dnd-kit/core';
import {
  arrayMove, SortableContext, sortableKeyboardCoordinates,
  useSortable, verticalListSortingStrategy,
} from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import { getRoutes, createRoute, updateRoute, deleteRoute, reorderRoutes } from '../api/client';
import type { Route } from '../types';

interface SortableRowProps {
  route: Route;
  onEdit: (route: Route) => void;
  onDelete: (id: string) => void;
}

function SortableRow({ route, onEdit, onDelete }: SortableRowProps) {
  const { attributes, listeners, setNodeRef, transform, transition } = useSortable({ id: route.id });
  const style = { transform: CSS.Transform.toString(transform), transition };

  return (
    <TableRow ref={setNodeRef} style={style} {...attributes}>
      <TableCell>
        <IconButton size="small" {...listeners} sx={{ cursor: 'grab' }}>
          <DragIcon />
        </IconButton>
      </TableCell>
      <TableCell><Typography variant="body1" fontWeight="bold">{route.name}</Typography></TableCell>
      <TableCell><Typography variant="body2" fontFamily="monospace">{route.pattern}</Typography></TableCell>
      <TableCell>{route.destination}</TableCell>
      <TableCell>
        <Chip label={route.enabled ? 'Enabled' : 'Disabled'} size="small" 
          color={route.enabled ? 'success' : 'default'} />
      </TableCell>
      <TableCell align="center">
        <Tooltip title="Edit Route">
          <IconButton size="small" color="primary" onClick={() => onEdit(route)}>
            <EditIcon />
          </IconButton>
        </Tooltip>
        <Tooltip title="Delete Route">
          <IconButton size="small" color="error" onClick={() => onDelete(route.id)}>
            <DeleteIcon />
          </IconButton>
        </Tooltip>
      </TableCell>
    </TableRow>
  );
}

export default function Routes() {
  const [items, setItems] = useState<Route[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editMode, setEditMode] = useState(false);
  const [current, setCurrent] = useState<Route>({
    id: '', name: '', description: '', pattern: '', destination: '',
    enabled: true, priority: 0,
  });

  const fetchItems = async () => {
    try {
      setLoading(true);
      const response = await getRoutes();
      setItems(response.routes);
      setError(null);
    } catch (err) {
      setError('Failed to fetch routes from server');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { fetchItems(); }, []);

  const handleOpenDialog = (item?: Route) => {
    if (item) {
      setCurrent(item);
      setEditMode(true);
    } else {
      setCurrent({
        id: `route-${Date.now()}`, name: '', description: '', pattern: '',
        destination: '', enabled: true, priority: items.length,
      });
      setEditMode(false);
    }
    setDialogOpen(true);
  };

  const handleSave = async () => {
    try {
      setError(null);
      setSuccess(null);
      if (editMode) {
        await updateRoute(current.id, current);
        setSuccess(`Route '${current.name}' updated successfully`);
      } else {
        await createRoute(current);
        setSuccess(`Route '${current.name}' created successfully`);
      }
      setDialogOpen(false);
      await fetchItems();
    } catch (err) {
      setError(`Failed to save route: ${err}`);
    }
  };

  const handleDelete = async (id: string) => {
    const item = items.find(i => i.id === id);
    if (!confirm(`Delete route '${item?.name}'?`)) return;
    try {
      setError(null);
      setSuccess(null);
      await deleteRoute(id);
      setSuccess('Route deleted successfully');
      await fetchItems();
    } catch (err) {
      setError('Failed to delete route');
    }
  };

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, { coordinateGetter: sortableKeyboardCoordinates })
  );

  const handleDragEnd = async (event: DragEndEvent) => {
    const { active, over } = event;
    if (over && active.id !== over.id) {
      const oldIndex = items.findIndex((i) => i.id === active.id);
      const newIndex = items.findIndex((i) => i.id === over.id);
      setItems((items) => arrayMove(items, oldIndex, newIndex));
      try {
        const result = await reorderRoutes({ from_index: oldIndex, to_index: newIndex });
        setItems(result.routes);
        setSuccess('Route order updated');
      } catch (err) {
        setError('Failed to reorder routes');
        await fetchItems();
      }
    }
  };

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
        <CircularProgress />
      </Box>
    );
  }

  return (
    <Box>
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="h4">Route/Dialplan Management</Typography>
        <Box>
          <Button variant="outlined" startIcon={<RefreshIcon />} onClick={fetchItems} sx={{ mr: 2 }}>
            Refresh
          </Button>
          <Button variant="contained" color="primary" startIcon={<AddIcon />} onClick={() => handleOpenDialog()}>
            Add Route
          </Button>
        </Box>
      </Box>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

      <Grid container spacing={3}>
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>Routing Rules</Typography>
              <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
                Manage routing rules and dialplans. Drag and drop to change priority order (higher priority routes are evaluated first).
              </Typography>
              <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
                <TableContainer component={Paper} variant="outlined">
                  <Table>
                    <TableHead>
                      <TableRow>
                        <TableCell width="50px"></TableCell>
                        <TableCell>Name</TableCell>
                        <TableCell>Pattern</TableCell>
                        <TableCell>Destination</TableCell>
                        <TableCell>Status</TableCell>
                        <TableCell align="center">Actions</TableCell>
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {items.length === 0 ? (
                        <TableRow>
                          <TableCell colSpan={6} align="center">
                            <Typography variant="body2" color="text.secondary">
                              No routes configured. Click "Add Route" to get started.
                            </Typography>
                          </TableCell>
                        </TableRow>
                      ) : (
                        <SortableContext items={items.map((i) => i.id)} strategy={verticalListSortingStrategy}>
                          {items.map((item) => (
                            <SortableRow key={item.id} route={item} onEdit={handleOpenDialog} onDelete={handleDelete} />
                          ))}
                        </SortableContext>
                      )}
                    </TableBody>
                  </Table>
                </TableContainer>
              </DndContext>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      <Dialog open={dialogOpen} onClose={() => setDialogOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>{editMode ? 'Edit Route' : 'Add Route'}</DialogTitle>
        <DialogContent>
          <Grid container spacing={2} sx={{ mt: 1 }}>
            <Grid item xs={12}>
              <TextField fullWidth label="Route Name" value={current.name}
                onChange={(e) => setCurrent({ ...current, name: e.target.value })}
                placeholder="e.g., Local Calls" required />
            </Grid>
            <Grid item xs={12}>
              <TextField fullWidth label="Pattern (regex)" value={current.pattern}
                onChange={(e) => setCurrent({ ...current, pattern: e.target.value })}
                placeholder="e.g., ^\d{4}$ for 4-digit extensions" required
                helperText="Regular expression to match dialed numbers" />
            </Grid>
            <Grid item xs={12}>
              <TextField fullWidth label="Destination" value={current.destination}
                onChange={(e) => setCurrent({ ...current, destination: e.target.value })}
                placeholder="e.g., extension:${1} or trunk:provider1" required
                helperText="Where to route matching calls (supports variables)" />
            </Grid>
            <Grid item xs={12}>
              <TextField fullWidth label="Description" value={current.description}
                onChange={(e) => setCurrent({ ...current, description: e.target.value })}
                multiline rows={2} placeholder="Optional description" />
            </Grid>
            <Grid item xs={12}>
              <FormControlLabel control={
                <Switch checked={current.enabled}
                  onChange={(e) => setCurrent({ ...current, enabled: e.target.checked })} />
              } label="Enabled" />
            </Grid>
          </Grid>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleSave} variant="contained" color="primary"
            disabled={!current.name || !current.pattern || !current.destination}>
            {editMode ? 'Update' : 'Create'}
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
