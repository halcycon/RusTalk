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
import { getTrunks, createTrunk, updateTrunk, deleteTrunk, reorderTrunks } from '../api/client';
import type { Trunk } from '../types';

interface SortableRowProps {
  trunk: Trunk;
  onEdit: (trunk: Trunk) => void;
  onDelete: (id: string) => void;
}

function SortableRow({ trunk, onEdit, onDelete }: SortableRowProps) {
  const { attributes, listeners, setNodeRef, transform, transition } = useSortable({ id: trunk.id });
  const style = { transform: CSS.Transform.toString(transform), transition };

  return (
    <TableRow ref={setNodeRef} style={style} {...attributes}>
      <TableCell>
        <IconButton size="small" {...listeners} sx={{ cursor: 'grab' }}>
          <DragIcon />
        </IconButton>
      </TableCell>
      <TableCell><Typography variant="body1" fontWeight="bold">{trunk.name}</Typography></TableCell>
      <TableCell>{trunk.host}:{trunk.port}</TableCell>
      <TableCell>{trunk.username || '-'}</TableCell>
      <TableCell>
        <Chip label={trunk.enabled ? 'Enabled' : 'Disabled'} size="small" 
          color={trunk.enabled ? 'success' : 'default'} />
      </TableCell>
      <TableCell align="center">
        <Tooltip title="Edit Trunk">
          <IconButton size="small" color="primary" onClick={() => onEdit(trunk)}>
            <EditIcon />
          </IconButton>
        </Tooltip>
        <Tooltip title="Delete Trunk">
          <IconButton size="small" color="error" onClick={() => onDelete(trunk.id)}>
            <DeleteIcon />
          </IconButton>
        </Tooltip>
      </TableCell>
    </TableRow>
  );
}

export default function Trunks() {
  const [items, setItems] = useState<Trunk[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editMode, setEditMode] = useState(false);
  const [current, setCurrent] = useState<Trunk>({
    id: '', name: '', description: '', host: '', port: 5060,
    username: '', password: '', enabled: true, priority: 0,
  });

  const fetchItems = async () => {
    try {
      setLoading(true);
      const response = await getTrunks();
      setItems(response.trunks);
      setError(null);
    } catch (err) {
      setError('Failed to fetch trunks from server');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { fetchItems(); }, []);

  const handleOpenDialog = (item?: Trunk) => {
    if (item) {
      setCurrent(item);
      setEditMode(true);
    } else {
      setCurrent({
        id: `trunk-${Date.now()}`, name: '', description: '', host: '', port: 5060,
        username: '', password: '', enabled: true, priority: items.length,
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
        await updateTrunk(current.id, current);
        setSuccess(`Trunk '${current.name}' updated successfully`);
      } else {
        await createTrunk(current);
        setSuccess(`Trunk '${current.name}' created successfully`);
      }
      setDialogOpen(false);
      await fetchItems();
    } catch (err) {
      setError(`Failed to save trunk: ${err}`);
    }
  };

  const handleDelete = async (id: string) => {
    const item = items.find(i => i.id === id);
    if (!confirm(`Delete trunk '${item?.name}'?`)) return;
    try {
      setError(null);
      setSuccess(null);
      await deleteTrunk(id);
      setSuccess('Trunk deleted successfully');
      await fetchItems();
    } catch (err) {
      setError('Failed to delete trunk');
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
        const result = await reorderTrunks({ from_index: oldIndex, to_index: newIndex });
        setItems(result.trunks);
        setSuccess('Trunk order updated');
      } catch (err) {
        setError('Failed to reorder trunks');
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
        <Typography variant="h4">Trunk Management</Typography>
        <Box>
          <Button variant="outlined" startIcon={<RefreshIcon />} onClick={fetchItems} sx={{ mr: 2 }}>
            Refresh
          </Button>
          <Button variant="contained" color="primary" startIcon={<AddIcon />} onClick={() => handleOpenDialog()}>
            Add Trunk
          </Button>
        </Box>
      </Box>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

      <Grid container spacing={3}>
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>SIP Trunks</Typography>
              <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
                Manage SIP trunks for outbound calling. Drag and drop to change priority order.
              </Typography>
              <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
                <TableContainer component={Paper} variant="outlined">
                  <Table>
                    <TableHead>
                      <TableRow>
                        <TableCell width="50px"></TableCell>
                        <TableCell>Name</TableCell>
                        <TableCell>Host:Port</TableCell>
                        <TableCell>Username</TableCell>
                        <TableCell>Status</TableCell>
                        <TableCell align="center">Actions</TableCell>
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {items.length === 0 ? (
                        <TableRow>
                          <TableCell colSpan={6} align="center">
                            <Typography variant="body2" color="text.secondary">
                              No trunks configured. Click "Add Trunk" to get started.
                            </Typography>
                          </TableCell>
                        </TableRow>
                      ) : (
                        <SortableContext items={items.map((i) => i.id)} strategy={verticalListSortingStrategy}>
                          {items.map((item) => (
                            <SortableRow key={item.id} trunk={item} onEdit={handleOpenDialog} onDelete={handleDelete} />
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
        <DialogTitle>{editMode ? 'Edit Trunk' : 'Add Trunk'}</DialogTitle>
        <DialogContent>
          <Grid container spacing={2} sx={{ mt: 1 }}>
            <Grid item xs={12}>
              <TextField fullWidth label="Trunk Name" value={current.name}
                onChange={(e) => setCurrent({ ...current, name: e.target.value })}
                placeholder="e.g., Provider1" required />
            </Grid>
            <Grid item xs={8}>
              <TextField fullWidth label="Host" value={current.host}
                onChange={(e) => setCurrent({ ...current, host: e.target.value })}
                placeholder="sip.provider.com" required />
            </Grid>
            <Grid item xs={4}>
              <TextField fullWidth label="Port" type="number" value={current.port}
                onChange={(e) => setCurrent({ ...current, port: parseInt(e.target.value) })}
                placeholder="5060" required />
            </Grid>
            <Grid item xs={12}>
              <TextField fullWidth label="Username" value={current.username}
                onChange={(e) => setCurrent({ ...current, username: e.target.value })}
                placeholder="Optional username" />
            </Grid>
            <Grid item xs={12}>
              <TextField fullWidth label="Password" type="password" value={current.password}
                onChange={(e) => setCurrent({ ...current, password: e.target.value })}
                placeholder="Optional password" />
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
            disabled={!current.name || !current.host}>
            {editMode ? 'Update' : 'Create'}
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
