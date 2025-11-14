import { useEffect, useState } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Button,
  Grid,
  Alert,
  CircularProgress,
  Switch,
  FormControlLabel,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Chip,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  IconButton,
  Tooltip,
} from '@mui/material';
import {
  Add as AddIcon,
  Delete as DeleteIcon,
  Refresh as RefreshIcon,
  Edit as EditIcon,
  DragIndicator as DragIcon,
} from '@mui/icons-material';
import {
  DndContext,
  closestCenter,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
} from '@dnd-kit/core';
import type { DragEndEvent } from '@dnd-kit/core';
import {
  arrayMove,
  SortableContext,
  sortableKeyboardCoordinates,
  useSortable,
  verticalListSortingStrategy,
} from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import { getDids, createDid, updateDid, deleteDid, reorderDids } from '../api/client';
import type { Did } from '../types';

// Sortable Row Component
interface SortableRowProps {
  did: Did;
  onEdit: (did: Did) => void;
  onDelete: (id: string) => void;
}

function SortableRow({ did, onEdit, onDelete }: SortableRowProps) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
  } = useSortable({ id: did.id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
  };

  return (
    <TableRow ref={setNodeRef} style={style} {...attributes}>
      <TableCell>
        <IconButton size="small" {...listeners} sx={{ cursor: 'grab' }}>
          <DragIcon />
        </IconButton>
      </TableCell>
      <TableCell>
        <Typography variant="body1" fontWeight="bold">
          {did.number}
        </Typography>
      </TableCell>
      <TableCell>
        <Typography variant="body2" color="text.secondary">
          {did.description || '-'}
        </Typography>
      </TableCell>
      <TableCell>{did.destination}</TableCell>
      <TableCell>
        <Chip
          label={did.enabled ? 'Enabled' : 'Disabled'}
          size="small"
          color={did.enabled ? 'success' : 'default'}
        />
      </TableCell>
      <TableCell align="center">
        <Tooltip title="Edit DID">
          <IconButton
            size="small"
            color="primary"
            onClick={() => onEdit(did)}
          >
            <EditIcon />
          </IconButton>
        </Tooltip>
        <Tooltip title="Delete DID">
          <IconButton
            size="small"
            color="error"
            onClick={() => onDelete(did.id)}
          >
            <DeleteIcon />
          </IconButton>
        </Tooltip>
      </TableCell>
    </TableRow>
  );
}

export default function DIDs() {
  const [dids, setDids] = useState<Did[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editMode, setEditMode] = useState(false);
  const [currentDid, setCurrentDid] = useState<Did>({
    id: '',
    number: '',
    description: '',
    destination: '',
    enabled: true,
    priority: 0,
  });

  const fetchDids = async () => {
    try {
      setLoading(true);
      const response = await getDids();
      setDids(response.dids);
      setError(null);
    } catch (err) {
      setError('Failed to fetch DIDs from server');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDids();
  }, []);

  const handleOpenDialog = (did?: Did) => {
    if (did) {
      setCurrentDid(did);
      setEditMode(true);
    } else {
      setCurrentDid({
        id: `did-${Date.now()}`,
        number: '',
        description: '',
        destination: '',
        enabled: true,
        priority: dids.length,
      });
      setEditMode(false);
    }
    setDialogOpen(true);
  };

  const handleCloseDialog = () => {
    setDialogOpen(false);
    setEditMode(false);
  };

  const handleSaveDid = async () => {
    try {
      setError(null);
      setSuccess(null);
      
      if (editMode) {
        await updateDid(currentDid.id, currentDid);
        setSuccess(`DID '${currentDid.number}' updated successfully`);
      } else {
        await createDid(currentDid);
        setSuccess(`DID '${currentDid.number}' created successfully`);
      }
      
      setDialogOpen(false);
      await fetchDids();
    } catch (err) {
      setError(`Failed to save DID: ${err}`);
      console.error(err);
    }
  };

  const handleDeleteDid = async (id: string) => {
    const did = dids.find(d => d.id === id);
    if (!confirm(`Are you sure you want to delete DID '${did?.number}'?`)) {
      return;
    }

    try {
      setError(null);
      setSuccess(null);
      await deleteDid(id);
      setSuccess(`DID deleted successfully`);
      await fetchDids();
    } catch (err) {
      setError(`Failed to delete DID`);
      console.error(err);
    }
  };

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  const handleDragEnd = async (event: DragEndEvent) => {
    const { active, over } = event;

    if (over && active.id !== over.id) {
      const oldIndex = dids.findIndex((did) => did.id === active.id);
      const newIndex = dids.findIndex((did) => did.id === over.id);

      // Optimistically update UI
      setDids((items) => arrayMove(items, oldIndex, newIndex));

      try {
        setError(null);
        setSuccess(null);
        const result = await reorderDids({ from_index: oldIndex, to_index: newIndex });
        setDids(result.dids);
        setSuccess('DID order updated successfully');
      } catch (err) {
        setError('Failed to reorder DIDs');
        console.error(err);
        // Revert on error
        await fetchDids();
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
        <Typography variant="h4">
          DID Management
        </Typography>
        <Box>
          <Button
            variant="outlined"
            startIcon={<RefreshIcon />}
            onClick={fetchDids}
            sx={{ mr: 2 }}
          >
            Refresh
          </Button>
          <Button
            variant="contained"
            color="primary"
            startIcon={<AddIcon />}
            onClick={() => handleOpenDialog()}
          >
            Add DID
          </Button>
        </Box>
      </Box>

      {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mb: 2 }}>{success}</Alert>}

      <Grid container spacing={3}>
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                DID Numbers
              </Typography>
              <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
                Manage Direct Inward Dialing (DID) numbers. Drag and drop to change priority order.
              </Typography>

              <DndContext
                sensors={sensors}
                collisionDetection={closestCenter}
                onDragEnd={handleDragEnd}
              >
                <TableContainer component={Paper} variant="outlined">
                  <Table>
                    <TableHead>
                      <TableRow>
                        <TableCell width="50px"></TableCell>
                        <TableCell>DID Number</TableCell>
                        <TableCell>Description</TableCell>
                        <TableCell>Destination</TableCell>
                        <TableCell>Status</TableCell>
                        <TableCell align="center">Actions</TableCell>
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {dids.length === 0 ? (
                        <TableRow>
                          <TableCell colSpan={6} align="center">
                            <Typography variant="body2" color="text.secondary">
                              No DIDs configured. Click "Add DID" to get started.
                            </Typography>
                          </TableCell>
                        </TableRow>
                      ) : (
                        <SortableContext
                          items={dids.map((d) => d.id)}
                          strategy={verticalListSortingStrategy}
                        >
                          {dids.map((did) => (
                            <SortableRow
                              key={did.id}
                              did={did}
                              onEdit={handleOpenDialog}
                              onDelete={handleDeleteDid}
                            />
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

      {/* Add/Edit Dialog */}
      <Dialog open={dialogOpen} onClose={handleCloseDialog} maxWidth="sm" fullWidth>
        <DialogTitle>{editMode ? 'Edit DID' : 'Add DID'}</DialogTitle>
        <DialogContent>
          <Grid container spacing={2} sx={{ mt: 1 }}>
            <Grid item xs={12}>
              <TextField
                fullWidth
                label="DID Number"
                value={currentDid.number}
                onChange={(e) => setCurrentDid({ ...currentDid, number: e.target.value })}
                placeholder="e.g., +1234567890"
                required
              />
            </Grid>
            <Grid item xs={12}>
              <TextField
                fullWidth
                label="Description"
                value={currentDid.description}
                onChange={(e) => setCurrentDid({ ...currentDid, description: e.target.value })}
                placeholder="Optional description"
                multiline
                rows={2}
              />
            </Grid>
            <Grid item xs={12}>
              <TextField
                fullWidth
                label="Destination"
                value={currentDid.destination}
                onChange={(e) => setCurrentDid({ ...currentDid, destination: e.target.value })}
                placeholder="e.g., extension:1000 or ring_group:sales"
                required
              />
            </Grid>
            <Grid item xs={12}>
              <FormControlLabel
                control={
                  <Switch
                    checked={currentDid.enabled}
                    onChange={(e) => setCurrentDid({ ...currentDid, enabled: e.target.checked })}
                  />
                }
                label="Enabled"
              />
            </Grid>
          </Grid>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCloseDialog}>Cancel</Button>
          <Button
            onClick={handleSaveDid}
            variant="contained"
            color="primary"
            disabled={!currentDid.number || !currentDid.destination}
          >
            {editMode ? 'Update' : 'Create'}
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
