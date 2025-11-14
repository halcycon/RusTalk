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
import { getCodecs, updateCodec, addCodec, removeCodec, reorderCodecs } from '../api/client';
import type { Codec, CodecAddRequest } from '../types';

// Sortable Row Component
interface SortableRowProps {
  codec: Codec;
  onToggle: (name: string, enabled: boolean) => void;
  onRemove: (name: string, isStandard: boolean) => void;
}

function SortableRow({ codec, onToggle, onRemove }: SortableRowProps) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
  } = useSortable({ id: codec.name });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
  };

  return (
    <TableRow ref={setNodeRef} style={style} {...attributes}>
      <TableCell>
        <Box display="flex" alignItems="center" gap={1}>
          <IconButton size="small" {...listeners} sx={{ cursor: 'grab' }}>
            <DragIcon />
          </IconButton>
          <Typography variant="body1" fontWeight="bold">
            {codec.name}
          </Typography>
        </Box>
      </TableCell>
      <TableCell>{codec.payload_type}</TableCell>
      <TableCell>{(codec.clock_rate / 1000).toFixed(0)} kHz</TableCell>
      <TableCell>{codec.channels}</TableCell>
      <TableCell>
        <Typography variant="body2" color="text.secondary">
          {codec.description}
        </Typography>
      </TableCell>
      <TableCell>
        <Chip
          label={codec.is_standard ? 'Standard' : 'Custom'}
          size="small"
          color={codec.is_standard ? 'primary' : 'secondary'}
        />
      </TableCell>
      <TableCell>
        <FormControlLabel
          control={
            <Switch
              checked={codec.enabled}
              onChange={(e) => onToggle(codec.name, e.target.checked)}
              color="primary"
            />
          }
          label={codec.enabled ? 'Enabled' : 'Disabled'}
        />
      </TableCell>
      <TableCell align="center">
        {!codec.is_standard && (
          <Tooltip title="Remove codec">
            <IconButton
              size="small"
              color="error"
              onClick={() => onRemove(codec.name, codec.is_standard)}
            >
              <DeleteIcon />
            </IconButton>
          </Tooltip>
        )}
      </TableCell>
    </TableRow>
  );
}

export default function Codecs() {
  const [codecs, setCodecs] = useState<Codec[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [addDialogOpen, setAddDialogOpen] = useState(false);
  const [newCodec, setNewCodec] = useState<CodecAddRequest>({
    name: '',
    payload_type: 96,
    clock_rate: 8000,
    channels: 1,
    description: '',
  });

  const fetchCodecs = async () => {
    try {
      setLoading(true);
      const response = await getCodecs();
      setCodecs(response.codecs);
      setError(null);
    } catch (err) {
      setError('Failed to fetch codecs from server');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchCodecs();
  }, []);

  const handleToggleCodec = async (name: string, enabled: boolean) => {
    try {
      setError(null);
      setSuccess(null);
      await updateCodec({ name, enabled });
      setSuccess(`Codec '${name}' ${enabled ? 'enabled' : 'disabled'} successfully`);
      await fetchCodecs();
    } catch (err) {
      setError(`Failed to update codec '${name}'`);
      console.error(err);
    }
  };

  const handleAddCodec = async () => {
    try {
      setError(null);
      setSuccess(null);
      await addCodec(newCodec);
      setSuccess(`Codec '${newCodec.name}' added successfully`);
      setAddDialogOpen(false);
      setNewCodec({
        name: '',
        payload_type: 96,
        clock_rate: 8000,
        channels: 1,
        description: '',
      });
      await fetchCodecs();
    } catch (err) {
      setError(`Failed to add codec: ${err}`);
      console.error(err);
    }
  };

  const handleRemoveCodec = async (name: string, isStandard: boolean) => {
    if (isStandard) {
      setError('Cannot remove standard codecs');
      return;
    }

    if (!confirm(`Are you sure you want to remove codec '${name}'?`)) {
      return;
    }

    try {
      setError(null);
      setSuccess(null);
      await removeCodec({ name });
      setSuccess(`Codec '${name}' removed successfully`);
      await fetchCodecs();
    } catch (err) {
      setError(`Failed to remove codec '${name}'`);
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
      const oldIndex = codecs.findIndex((codec) => codec.name === active.id);
      const newIndex = codecs.findIndex((codec) => codec.name === over.id);

      // Optimistically update UI
      setCodecs((items) => arrayMove(items, oldIndex, newIndex));

      try {
        setError(null);
        setSuccess(null);
        const result = await reorderCodecs({ from_index: oldIndex, to_index: newIndex });
        setCodecs(result.codecs);
        setSuccess('Codec order updated successfully');
      } catch (err) {
        setError('Failed to reorder codecs');
        console.error(err);
        // Revert on error
        await fetchCodecs();
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
          Codec Management
        </Typography>
        <Box>
          <Button
            variant="outlined"
            startIcon={<RefreshIcon />}
            onClick={fetchCodecs}
            sx={{ mr: 2 }}
          >
            Refresh
          </Button>
          <Button
            variant="contained"
            color="primary"
            startIcon={<AddIcon />}
            onClick={() => setAddDialogOpen(true)}
          >
            Add Custom Codec
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
                Available Codecs
              </Typography>
              <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
                Drag and drop to reorder codec priority. Enable or disable audio codecs for SIP calls. Standard codecs cannot be removed, but custom codecs can be added and removed.
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
                        <TableCell>Codec Name</TableCell>
                        <TableCell>Payload Type</TableCell>
                        <TableCell>Clock Rate</TableCell>
                        <TableCell>Channels</TableCell>
                        <TableCell>Description</TableCell>
                        <TableCell>Type</TableCell>
                        <TableCell>Status</TableCell>
                        <TableCell align="center">Actions</TableCell>
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      <SortableContext
                        items={codecs.map((c) => c.name)}
                        strategy={verticalListSortingStrategy}
                      >
                        {codecs.map((codec) => (
                          <SortableRow
                            key={codec.name}
                            codec={codec}
                            onToggle={handleToggleCodec}
                            onRemove={handleRemoveCodec}
                          />
                        ))}
                      </SortableContext>
                    </TableBody>
                  </Table>
                </TableContainer>
              </DndContext>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Add Codec Dialog */}
      <Dialog open={addDialogOpen} onClose={() => setAddDialogOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Add Custom Codec</DialogTitle>
        <DialogContent>
          <Grid container spacing={2} sx={{ mt: 1 }}>
            <Grid item xs={12}>
              <TextField
                fullWidth
                label="Codec Name"
                value={newCodec.name}
                onChange={(e) => setNewCodec({ ...newCodec, name: e.target.value })}
                placeholder="e.g., CustomCodec"
              />
            </Grid>
            <Grid item xs={12} md={6}>
              <TextField
                fullWidth
                label="Payload Type"
                type="number"
                value={newCodec.payload_type}
                onChange={(e) =>
                  setNewCodec({ ...newCodec, payload_type: parseInt(e.target.value) })
                }
                inputProps={{ min: 96, max: 127 }}
                helperText="Dynamic range: 96-127"
              />
            </Grid>
            <Grid item xs={12} md={6}>
              <TextField
                fullWidth
                label="Clock Rate (Hz)"
                type="number"
                value={newCodec.clock_rate}
                onChange={(e) =>
                  setNewCodec({ ...newCodec, clock_rate: parseInt(e.target.value) })
                }
                placeholder="8000"
                helperText="Common: 8000, 16000, 48000"
              />
            </Grid>
            <Grid item xs={12} md={6}>
              <TextField
                fullWidth
                label="Channels"
                type="number"
                value={newCodec.channels}
                onChange={(e) =>
                  setNewCodec({ ...newCodec, channels: parseInt(e.target.value) })
                }
                inputProps={{ min: 1, max: 2 }}
                helperText="1 (mono) or 2 (stereo)"
              />
            </Grid>
            <Grid item xs={12}>
              <TextField
                fullWidth
                label="Description"
                value={newCodec.description}
                onChange={(e) => setNewCodec({ ...newCodec, description: e.target.value })}
                placeholder="Brief description of the codec"
                multiline
                rows={2}
              />
            </Grid>
          </Grid>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setAddDialogOpen(false)}>Cancel</Button>
          <Button
            onClick={handleAddCodec}
            variant="contained"
            color="primary"
            disabled={!newCodec.name || !newCodec.description}
          >
            Add Codec
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
