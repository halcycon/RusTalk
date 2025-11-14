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
import { getExtensions, createExtension, updateExtension, deleteExtension, reorderExtensions } from '../api/client';
import type { Extension } from '../types';

interface SortableRowProps {
  ext: Extension;
  onEdit: (ext: Extension) => void;
  onDelete: (id: string) => void;
}

function SortableRow({ ext, onEdit, onDelete }: SortableRowProps) {
  const { attributes, listeners, setNodeRef, transform, transition } = useSortable({ id: ext.id });
  const style = { transform: CSS.Transform.toString(transform), transition };

  return (
    <TableRow ref={setNodeRef} style={style} {...attributes}>
      <TableCell>
        <IconButton size="small" {...listeners} sx={{ cursor: 'grab' }}>
          <DragIcon />
        </IconButton>
      </TableCell>
      <TableCell><Typography variant="body1" fontWeight="bold">{ext.extension}</Typography></TableCell>
      <TableCell>{ext.display_name}</TableCell>
      <TableCell>
        <Chip label={ext.voicemail_enabled ? 'Enabled' : 'Disabled'} size="small" 
          color={ext.voicemail_enabled ? 'success' : 'default'} />
      </TableCell>
      <TableCell>
        <Chip label={ext.enabled ? 'Enabled' : 'Disabled'} size="small" 
          color={ext.enabled ? 'success' : 'default'} />
      </TableCell>
      <TableCell align="center">
        <Tooltip title="Edit Extension">
          <IconButton size="small" color="primary" onClick={() => onEdit(ext)}>
            <EditIcon />
          </IconButton>
        </Tooltip>
        <Tooltip title="Delete Extension">
          <IconButton size="small" color="error" onClick={() => onDelete(ext.id)}>
            <DeleteIcon />
          </IconButton>
        </Tooltip>
      </TableCell>
    </TableRow>
  );
}

export default function Extensions() {
  const [items, setItems] = useState<Extension[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editMode, setEditMode] = useState(false);
  const [current, setCurrent] = useState<Extension>({
    id: '', extension: '', display_name: '', password: '', enabled: true,
    voicemail_enabled: false, priority: 0,
  });

  const fetchItems = async () => {
    try {
      setLoading(true);
      const response = await getExtensions();
      setItems(response.extensions);
      setError(null);
    } catch (err) {
      setError('Failed to fetch extensions from server');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { fetchItems(); }, []);

  const handleOpenDialog = (item?: Extension) => {
    if (item) {
      setCurrent(item);
      setEditMode(true);
    } else {
      setCurrent({
        id: `ext-${Date.now()}`, extension: '', display_name: '', password: '',
        enabled: true, voicemail_enabled: false, priority: items.length,
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
        await updateExtension(current.id, current);
        setSuccess(`Extension '${current.extension}' updated successfully`);
      } else {
        await createExtension(current);
        setSuccess(`Extension '${current.extension}' created successfully`);
      }
      setDialogOpen(false);
      await fetchItems();
    } catch (err) {
      setError(`Failed to save extension: ${err}`);
    }
  };

  const handleDelete = async (id: string) => {
    const item = items.find(i => i.id === id);
    if (!confirm(`Delete extension '${item?.extension}'?`)) return;
    try {
      setError(null);
      setSuccess(null);
      await deleteExtension(id);
      setSuccess('Extension deleted successfully');
      await fetchItems();
    } catch (err) {
      setError('Failed to delete extension');
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
        const result = await reorderExtensions({ from_index: oldIndex, to_index: newIndex });
        setItems(result.extensions);
        setSuccess('Extension order updated');
      } catch (err) {
        setError('Failed to reorder extensions');
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
        <Typography variant="h4">Extension/Endpoint Management</Typography>
        <Box>
          <Button variant="outlined" startIcon={<RefreshIcon />} onClick={fetchItems} sx={{ mr: 2 }}>
            Refresh
          </Button>
          <Button variant="contained" color="primary" startIcon={<AddIcon />} onClick={() => handleOpenDialog()}>
            Add Extension
          </Button>
        </Box>
      </Box>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

      <Grid container spacing={3}>
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>Extensions</Typography>
              <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
                Manage SIP extensions and endpoints. Drag and drop to change priority order.
              </Typography>
              <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
                <TableContainer component={Paper} variant="outlined">
                  <Table>
                    <TableHead>
                      <TableRow>
                        <TableCell width="50px"></TableCell>
                        <TableCell>Extension</TableCell>
                        <TableCell>Display Name</TableCell>
                        <TableCell>Voicemail</TableCell>
                        <TableCell>Status</TableCell>
                        <TableCell align="center">Actions</TableCell>
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {items.length === 0 ? (
                        <TableRow>
                          <TableCell colSpan={6} align="center">
                            <Typography variant="body2" color="text.secondary">
                              No extensions configured. Click "Add Extension" to get started.
                            </Typography>
                          </TableCell>
                        </TableRow>
                      ) : (
                        <SortableContext items={items.map((i) => i.id)} strategy={verticalListSortingStrategy}>
                          {items.map((item) => (
                            <SortableRow key={item.id} ext={item} onEdit={handleOpenDialog} onDelete={handleDelete} />
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
        <DialogTitle>{editMode ? 'Edit Extension' : 'Add Extension'}</DialogTitle>
        <DialogContent>
          <Grid container spacing={2} sx={{ mt: 1 }}>
            <Grid item xs={12}>
              <TextField fullWidth label="Extension Number" value={current.extension}
                onChange={(e) => setCurrent({ ...current, extension: e.target.value })}
                placeholder="e.g., 1000" required />
            </Grid>
            <Grid item xs={12}>
              <TextField fullWidth label="Display Name" value={current.display_name}
                onChange={(e) => setCurrent({ ...current, display_name: e.target.value })}
                placeholder="User name" required />
            </Grid>
            <Grid item xs={12}>
              <TextField fullWidth label="Password" type="password" value={current.password}
                onChange={(e) => setCurrent({ ...current, password: e.target.value })}
                placeholder="SIP password" required />
            </Grid>
            <Grid item xs={12}>
              <FormControlLabel control={
                <Switch checked={current.voicemail_enabled}
                  onChange={(e) => setCurrent({ ...current, voicemail_enabled: e.target.checked })} />
              } label="Voicemail Enabled" />
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
            disabled={!current.extension || !current.display_name || !current.password}>
            {editMode ? 'Update' : 'Create'}
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
