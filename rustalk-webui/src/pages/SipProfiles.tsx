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
import { getSipProfiles, createSipProfile, updateSipProfile, deleteSipProfile, reorderSipProfiles } from '../api/client';
import type { SipProfile } from '../types';

interface SortableRowProps {
  profile: SipProfile;
  onEdit: (profile: SipProfile) => void;
  onDelete: (id: string) => void;
}

function SortableRow({ profile, onEdit, onDelete }: SortableRowProps) {
  const { attributes, listeners, setNodeRef, transform, transition } = useSortable({ id: profile.id });
  const style = { transform: CSS.Transform.toString(transform), transition };

  return (
    <TableRow ref={setNodeRef} style={style} {...attributes}>
      <TableCell>
        <IconButton size="small" {...listeners} sx={{ cursor: 'grab' }}>
          <DragIcon />
        </IconButton>
      </TableCell>
      <TableCell><Typography variant="body1" fontWeight="bold">{profile.name}</Typography></TableCell>
      <TableCell>{profile.domain}</TableCell>
      <TableCell>{profile.bind_address}:{profile.bind_port}</TableCell>
      <TableCell>
        <Chip label={profile.enabled ? 'Enabled' : 'Disabled'} size="small" 
          color={profile.enabled ? 'success' : 'default'} />
      </TableCell>
      <TableCell align="center">
        <Tooltip title="Edit SIP Profile">
          <IconButton size="small" color="primary" onClick={() => onEdit(profile)}>
            <EditIcon />
          </IconButton>
        </Tooltip>
        <Tooltip title="Delete SIP Profile">
          <IconButton size="small" color="error" onClick={() => onDelete(profile.id)}>
            <DeleteIcon />
          </IconButton>
        </Tooltip>
      </TableCell>
    </TableRow>
  );
}

export default function SipProfiles() {
  const [items, setItems] = useState<SipProfile[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editMode, setEditMode] = useState(false);
  const [current, setCurrent] = useState<SipProfile>({
    id: '', name: '', description: '', bind_address: '0.0.0.0', bind_port: 5060,
    domain: '', enabled: true, priority: 0,
  });

  const fetchItems = async () => {
    try {
      setLoading(true);
      const response = await getSipProfiles();
      setItems(response.sip_profiles);
      setError(null);
    } catch (err) {
      setError('Failed to fetch SIP profiles from server');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { fetchItems(); }, []);

  const handleOpenDialog = (item?: SipProfile) => {
    if (item) {
      setCurrent(item);
      setEditMode(true);
    } else {
      setCurrent({
        id: `profile-${Date.now()}`, name: '', description: '', bind_address: '0.0.0.0',
        bind_port: 5060, domain: '', enabled: true, priority: items.length,
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
        await updateSipProfile(current.id, current);
        setSuccess(`SIP profile '${current.name}' updated successfully`);
      } else {
        await createSipProfile(current);
        setSuccess(`SIP profile '${current.name}' created successfully`);
      }
      setDialogOpen(false);
      await fetchItems();
    } catch (err) {
      setError(`Failed to save SIP profile: ${err}`);
    }
  };

  const handleDelete = async (id: string) => {
    const item = items.find(i => i.id === id);
    if (!confirm(`Delete SIP profile '${item?.name}'?`)) return;
    try {
      setError(null);
      setSuccess(null);
      await deleteSipProfile(id);
      setSuccess('SIP profile deleted successfully');
      await fetchItems();
    } catch (err) {
      setError('Failed to delete SIP profile');
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
        const result = await reorderSipProfiles({ from_index: oldIndex, to_index: newIndex });
        setItems(result.sip_profiles);
        setSuccess('SIP profile order updated');
      } catch (err) {
        setError('Failed to reorder SIP profiles');
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
        <Typography variant="h4">SIP Profile Management</Typography>
        <Box>
          <Button variant="outlined" startIcon={<RefreshIcon />} onClick={fetchItems} sx={{ mr: 2 }}>
            Refresh
          </Button>
          <Button variant="contained" color="primary" startIcon={<AddIcon />} onClick={() => handleOpenDialog()}>
            Add SIP Profile
          </Button>
        </Box>
      </Box>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

      <Grid container spacing={3}>
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>SIP Profiles</Typography>
              <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
                Manage SIP profiles for different network interfaces. Drag and drop to change priority order.
              </Typography>
              <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
                <TableContainer component={Paper} variant="outlined">
                  <Table>
                    <TableHead>
                      <TableRow>
                        <TableCell width="50px"></TableCell>
                        <TableCell>Name</TableCell>
                        <TableCell>Domain</TableCell>
                        <TableCell>Bind Address:Port</TableCell>
                        <TableCell>Status</TableCell>
                        <TableCell align="center">Actions</TableCell>
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {items.length === 0 ? (
                        <TableRow>
                          <TableCell colSpan={6} align="center">
                            <Typography variant="body2" color="text.secondary">
                              No SIP profiles configured. Click "Add SIP Profile" to get started.
                            </Typography>
                          </TableCell>
                        </TableRow>
                      ) : (
                        <SortableContext items={items.map((i) => i.id)} strategy={verticalListSortingStrategy}>
                          {items.map((item) => (
                            <SortableRow key={item.id} profile={item} onEdit={handleOpenDialog} onDelete={handleDelete} />
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
        <DialogTitle>{editMode ? 'Edit SIP Profile' : 'Add SIP Profile'}</DialogTitle>
        <DialogContent>
          <Grid container spacing={2} sx={{ mt: 1 }}>
            <Grid item xs={12}>
              <TextField fullWidth label="Profile Name" value={current.name}
                onChange={(e) => setCurrent({ ...current, name: e.target.value })}
                placeholder="e.g., internal" required />
            </Grid>
            <Grid item xs={12}>
              <TextField fullWidth label="Domain" value={current.domain}
                onChange={(e) => setCurrent({ ...current, domain: e.target.value })}
                placeholder="e.g., sip.local" required />
            </Grid>
            <Grid item xs={8}>
              <TextField fullWidth label="Bind Address" value={current.bind_address}
                onChange={(e) => setCurrent({ ...current, bind_address: e.target.value })}
                placeholder="0.0.0.0" required />
            </Grid>
            <Grid item xs={4}>
              <TextField fullWidth label="Port" type="number" value={current.bind_port}
                onChange={(e) => setCurrent({ ...current, bind_port: parseInt(e.target.value) })}
                placeholder="5060" required />
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
            disabled={!current.name || !current.domain || !current.bind_address}>
            {editMode ? 'Update' : 'Create'}
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
