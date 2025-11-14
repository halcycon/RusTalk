import { useEffect, useState } from 'react';
import {
  Box, Card, CardContent, Typography, Button, Grid, Alert, CircularProgress,
  Switch, FormControlLabel, Table, TableBody, TableCell, TableContainer, TableHead,
  TableRow, Paper, Chip, Dialog, DialogTitle, DialogContent, DialogActions,
  TextField, IconButton, Tooltip, Select, MenuItem, FormControl, InputLabel,
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
import { getRingGroups, createRingGroup, updateRingGroup, deleteRingGroup, reorderRingGroups } from '../api/client';
import type { RingGroup } from '../types';

interface SortableRowProps {
  group: RingGroup;
  onEdit: (group: RingGroup) => void;
  onDelete: (id: string) => void;
}

function SortableRow({ group, onEdit, onDelete }: SortableRowProps) {
  const { attributes, listeners, setNodeRef, transform, transition } = useSortable({ id: group.id });
  const style = { transform: CSS.Transform.toString(transform), transition };

  return (
    <TableRow ref={setNodeRef} style={style} {...attributes}>
      <TableCell>
        <IconButton size="small" {...listeners} sx={{ cursor: 'grab' }}>
          <DragIcon />
        </IconButton>
      </TableCell>
      <TableCell><Typography variant="body1" fontWeight="bold">{group.name}</Typography></TableCell>
      <TableCell><Chip label={group.strategy} size="small" color="primary" /></TableCell>
      <TableCell>{group.extensions.join(', ')}</TableCell>
      <TableCell>
        <Chip label={group.enabled ? 'Enabled' : 'Disabled'} size="small" 
          color={group.enabled ? 'success' : 'default'} />
      </TableCell>
      <TableCell align="center">
        <Tooltip title="Edit Ring Group">
          <IconButton size="small" color="primary" onClick={() => onEdit(group)}>
            <EditIcon />
          </IconButton>
        </Tooltip>
        <Tooltip title="Delete Ring Group">
          <IconButton size="small" color="error" onClick={() => onDelete(group.id)}>
            <DeleteIcon />
          </IconButton>
        </Tooltip>
      </TableCell>
    </TableRow>
  );
}

export default function RingGroups() {
  const [items, setItems] = useState<RingGroup[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editMode, setEditMode] = useState(false);
  const [current, setCurrent] = useState<RingGroup>({
    id: '', name: '', description: '', extensions: [], strategy: 'simultaneous',
    timeout_seconds: 30, enabled: true, priority: 0,
  });

  const fetchItems = async () => {
    try {
      setLoading(true);
      const response = await getRingGroups();
      setItems(response.ring_groups);
      setError(null);
    } catch (err) {
      setError('Failed to fetch ring groups from server');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { fetchItems(); }, []);

  const handleOpenDialog = (item?: RingGroup) => {
    if (item) {
      setCurrent(item);
      setEditMode(true);
    } else {
      setCurrent({
        id: `rg-${Date.now()}`, name: '', description: '', extensions: [],
        strategy: 'simultaneous', timeout_seconds: 30, enabled: true, priority: items.length,
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
        await updateRingGroup(current.id, current);
        setSuccess(`Ring group '${current.name}' updated successfully`);
      } else {
        await createRingGroup(current);
        setSuccess(`Ring group '${current.name}' created successfully`);
      }
      setDialogOpen(false);
      await fetchItems();
    } catch (err) {
      setError(`Failed to save ring group: ${err}`);
    }
  };

  const handleDelete = async (id: string) => {
    const item = items.find(i => i.id === id);
    if (!confirm(`Delete ring group '${item?.name}'?`)) return;
    try {
      setError(null);
      setSuccess(null);
      await deleteRingGroup(id);
      setSuccess('Ring group deleted successfully');
      await fetchItems();
    } catch (err) {
      setError('Failed to delete ring group');
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
        const result = await reorderRingGroups({ from_index: oldIndex, to_index: newIndex });
        setItems(result.ring_groups);
        setSuccess('Ring group order updated');
      } catch (err) {
        setError('Failed to reorder ring groups');
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
        <Typography variant="h4">Ring Group Management</Typography>
        <Box>
          <Button variant="outlined" startIcon={<RefreshIcon />} onClick={fetchItems} sx={{ mr: 2 }}>
            Refresh
          </Button>
          <Button variant="contained" color="primary" startIcon={<AddIcon />} onClick={() => handleOpenDialog()}>
            Add Ring Group
          </Button>
        </Box>
      </Box>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

      <Grid container spacing={3}>
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>Ring Groups</Typography>
              <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
                Manage ring groups for call distribution. Drag and drop to change priority order.
              </Typography>
              <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
                <TableContainer component={Paper} variant="outlined">
                  <Table>
                    <TableHead>
                      <TableRow>
                        <TableCell width="50px"></TableCell>
                        <TableCell>Name</TableCell>
                        <TableCell>Strategy</TableCell>
                        <TableCell>Extensions</TableCell>
                        <TableCell>Status</TableCell>
                        <TableCell align="center">Actions</TableCell>
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {items.length === 0 ? (
                        <TableRow>
                          <TableCell colSpan={6} align="center">
                            <Typography variant="body2" color="text.secondary">
                              No ring groups configured. Click "Add Ring Group" to get started.
                            </Typography>
                          </TableCell>
                        </TableRow>
                      ) : (
                        <SortableContext items={items.map((i) => i.id)} strategy={verticalListSortingStrategy}>
                          {items.map((item) => (
                            <SortableRow key={item.id} group={item} onEdit={handleOpenDialog} onDelete={handleDelete} />
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
        <DialogTitle>{editMode ? 'Edit Ring Group' : 'Add Ring Group'}</DialogTitle>
        <DialogContent>
          <Grid container spacing={2} sx={{ mt: 1 }}>
            <Grid item xs={12}>
              <TextField fullWidth label="Group Name" value={current.name}
                onChange={(e) => setCurrent({ ...current, name: e.target.value })}
                placeholder="e.g., Sales Team" required />
            </Grid>
            <Grid item xs={12}>
              <FormControl fullWidth>
                <InputLabel>Ring Strategy</InputLabel>
                <Select value={current.strategy}
                  onChange={(e) => setCurrent({ ...current, strategy: e.target.value as any })}
                  label="Ring Strategy">
                  <MenuItem value="simultaneous">Simultaneous (ring all at once)</MenuItem>
                  <MenuItem value="sequential">Sequential (ring in order)</MenuItem>
                  <MenuItem value="roundrobin">Round Robin (distribute evenly)</MenuItem>
                </Select>
              </FormControl>
            </Grid>
            <Grid item xs={12}>
              <TextField fullWidth label="Timeout (seconds)" type="number" value={current.timeout_seconds}
                onChange={(e) => setCurrent({ ...current, timeout_seconds: parseInt(e.target.value) })}
                placeholder="30" required />
            </Grid>
            <Grid item xs={12}>
              <TextField fullWidth label="Extensions (comma-separated)"
                value={current.extensions.join(',')}
                onChange={(e) => setCurrent({ ...current, extensions: e.target.value.split(',').map(s => s.trim()).filter(s => s) })}
                placeholder="1000,1001,1002" required />
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
            disabled={!current.name || current.extensions.length === 0}>
            {editMode ? 'Update' : 'Create'}
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
