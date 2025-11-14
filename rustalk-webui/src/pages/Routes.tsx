import { useEffect, useState } from 'react';
import {
  Box, Card, CardContent, Typography, Button, Grid, Alert, CircularProgress,
  Switch, FormControlLabel, Table, TableBody, TableCell, TableContainer, TableHead,
  TableRow, Paper, Chip, Dialog, DialogTitle, DialogContent, DialogActions,
  TextField, IconButton, Tooltip, Select, MenuItem, FormControl, InputLabel,
  Accordion, AccordionSummary, AccordionDetails, Checkbox, FormGroup,
  Stack, Divider,
} from '@mui/material';
import {
  Add as AddIcon, Delete as DeleteIcon, Refresh as RefreshIcon,
  Edit as EditIcon, DragIndicator as DragIcon, ExpandMore as ExpandMoreIcon,
  PlayArrow as TestIcon, Close as CloseIcon,
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
import { getRoutes, createRoute, updateRoute, deleteRoute, reorderRoutes, testRoute } from '../api/client';
import type { Route, RouteDestination, RouteCondition, RouteActionType, TimeCondition, DayOfWeekCondition, DateRangeCondition, CallerIdCondition, DestinationCondition } from '../types';

interface SortableRowProps {
  route: Route;
  onEdit: (route: Route) => void;
  onDelete: (id: string) => void;
}

function SortableRow({ route, onEdit, onDelete }: SortableRowProps) {
  const { attributes, listeners, setNodeRef, transform, transition } = useSortable({ id: route.id });
  const style = { transform: CSS.Transform.toString(transform), transition };

  const getDestinationDisplay = (dest: RouteDestination | string): string => {
    if (typeof dest === 'string') return dest;
    return dest.type === 'Hangup' ? 'Hangup' : `${dest.type}: ${dest.value}`;
  };

  return (
    <TableRow ref={setNodeRef} style={style} {...attributes}>
      <TableCell>
        <IconButton size="small" {...listeners} sx={{ cursor: 'grab' }}>
          <DragIcon />
        </IconButton>
      </TableCell>
      <TableCell><Typography variant="body1" fontWeight="bold">{route.name}</Typography></TableCell>
      <TableCell><Typography variant="body2" fontFamily="monospace">{route.pattern}</Typography></TableCell>
      <TableCell>{getDestinationDisplay(route.destination)}</TableCell>
      <TableCell>
        <Chip label={route.action} size="small" color={route.action === 'accept' ? 'success' : route.action === 'reject' ? 'error' : 'default'} />
      </TableCell>
      <TableCell>
        <Chip label={route.enabled ? 'Enabled' : 'Disabled'} size="small" 
          color={route.enabled ? 'success' : 'default'} />
      </TableCell>
      <TableCell>
        {route.conditions && route.conditions.length > 0 ? (
          <Chip label={`${route.conditions.length} condition${route.conditions.length > 1 ? 's' : ''}`} size="small" color="info" />
        ) : (
          <Typography variant="body2" color="text.secondary">None</Typography>
        )}
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

export default function RoutesNew() {
  const [items, setItems] = useState<Route[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);
  const [testDialogOpen, setTestDialogOpen] = useState(false);
  const [editMode, setEditMode] = useState(false);
  const [testCallerId, setTestCallerId] = useState('+12125551234');
  const [testDestination, setTestDestination] = useState('2345');
  const [testResult, setTestResult] = useState<any>(null);
  
  const [current, setCurrent] = useState<Route>({
    id: '', name: '', description: '', pattern: '', 
    destination: { type: 'Extension', value: '' },
    enabled: true, priority: 0, conditions: [], action: 'accept', continue_on_match: false,
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
        destination: { type: 'Extension', value: '' }, enabled: true, priority: items.length,
        conditions: [], action: 'accept', continue_on_match: false,
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

  const handleTest = async () => {
    try {
      setTestResult(null);
      const result = await testRoute({
        caller_id: testCallerId,
        destination: testDestination,
      });
      setTestResult(result);
    } catch (err) {
      setError('Failed to test route');
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

  const addCondition = (type: string) => {
    const newCondition: RouteCondition = 
      type === 'Time' ? { type: 'Time', start_time: '09:00', end_time: '17:00' } :
      type === 'DayOfWeek' ? { type: 'DayOfWeek', days: [1, 2, 3, 4, 5] } :
      type === 'DateRange' ? { type: 'DateRange', start_date: '', end_date: '' } :
      type === 'CallerId' ? { type: 'CallerId', pattern: '', negate: false } :
      { type: 'Destination', pattern: '', negate: false };
    
    setCurrent({ ...current, conditions: [...(current.conditions || []), newCondition] });
  };

  const removeCondition = (index: number) => {
    const newConditions = [...(current.conditions || [])];
    newConditions.splice(index, 1);
    setCurrent({ ...current, conditions: newConditions });
  };

  const updateCondition = (index: number, updatedCondition: RouteCondition) => {
    const newConditions = [...(current.conditions || [])];
    newConditions[index] = updatedCondition;
    setCurrent({ ...current, conditions: newConditions });
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
        <Typography variant="h4">Advanced Routing Rules</Typography>
        <Box>
          <Button variant="outlined" startIcon={<TestIcon />} onClick={() => setTestDialogOpen(true)} sx={{ mr: 2 }}>
            Test Routes
          </Button>
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
                Routes are evaluated in priority order (top to bottom). Drag and drop to reorder. Routes support advanced conditions like time-of-day, day-of-week, caller ID filtering, and more.
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
                        <TableCell>Action</TableCell>
                        <TableCell>Status</TableCell>
                        <TableCell>Conditions</TableCell>
                        <TableCell align="center">Actions</TableCell>
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {items.length === 0 ? (
                        <TableRow>
                          <TableCell colSpan={8} align="center">
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

      {/* Edit/Create Dialog */}
      <Dialog open={dialogOpen} onClose={() => setDialogOpen(false)} maxWidth="md" fullWidth>
        <DialogTitle>{editMode ? 'Edit Route' : 'Add Route'}</DialogTitle>
        <DialogContent>
          <Grid container spacing={2} sx={{ mt: 1 }}>
            <Grid item xs={12}>
              <TextField fullWidth label="Route Name" value={current.name}
                onChange={(e) => setCurrent({ ...current, name: e.target.value })}
                placeholder="e.g., Business Hours Extensions" required />
            </Grid>
            <Grid item xs={12}>
              <TextField fullWidth label="Pattern (regex)" value={current.pattern}
                onChange={(e) => setCurrent({ ...current, pattern: e.target.value })}
                placeholder="e.g., ^[2-5]\d{3}$ for extensions 2000-5999" required
                helperText="Regular expression to match dialed numbers" />
            </Grid>
            
            <Grid item xs={12} sm={6}>
              <FormControl fullWidth>
                <InputLabel>Destination Type</InputLabel>
                <Select
                  value={typeof current.destination === 'string' ? 'Custom' : current.destination.type}
                  onChange={(e) => {
                    const type = e.target.value;
                    setCurrent({ 
                      ...current, 
                      destination: type === 'Hangup' ? { type: 'Hangup' } : { type: type as any, value: '' }
                    });
                  }}
                >
                  <MenuItem value="Extension">Extension</MenuItem>
                  <MenuItem value="Trunk">Trunk</MenuItem>
                  <MenuItem value="RingGroup">Ring Group</MenuItem>
                  <MenuItem value="Voicemail">Voicemail</MenuItem>
                  <MenuItem value="Hangup">Hangup</MenuItem>
                  <MenuItem value="Custom">Custom</MenuItem>
                </Select>
              </FormControl>
            </Grid>
            
            {typeof current.destination !== 'string' && current.destination.type !== 'Hangup' && (
              <Grid item xs={12} sm={6}>
                <TextField fullWidth label="Destination Value"
                  value={(typeof current.destination !== 'string' && current.destination.value) || ''}
                  onChange={(e) => {
                    if (typeof current.destination !== 'string') {
                      setCurrent({ 
                        ...current, 
                        destination: { ...current.destination, value: e.target.value }
                      });
                    }
                  }}
                  placeholder="e.g., 1000 or trunk-name" required />
              </Grid>
            )}

            <Grid item xs={12} sm={6}>
              <FormControl fullWidth>
                <InputLabel>Action</InputLabel>
                <Select
                  value={current.action}
                  onChange={(e) => setCurrent({ ...current, action: e.target.value as RouteActionType })}
                >
                  <MenuItem value="accept">Accept</MenuItem>
                  <MenuItem value="reject">Reject</MenuItem>
                  <MenuItem value="continue">Continue</MenuItem>
                </Select>
              </FormControl>
            </Grid>

            <Grid item xs={12} sm={6}>
              <FormControlLabel control={
                <Switch checked={current.continue_on_match}
                  onChange={(e) => setCurrent({ ...current, continue_on_match: e.target.checked })} />
              } label="Continue on Match" />
            </Grid>
            
            <Grid item xs={12}>
              <TextField fullWidth label="Description" value={current.description || ''}
                onChange={(e) => setCurrent({ ...current, description: e.target.value })}
                multiline rows={2} placeholder="Optional description" />
            </Grid>
            
            <Grid item xs={12}>
              <FormControlLabel control={
                <Switch checked={current.enabled}
                  onChange={(e) => setCurrent({ ...current, enabled: e.target.checked })} />
              } label="Enabled" />
            </Grid>

            <Grid item xs={12}>
              <Divider sx={{ my: 2 }} />
              <Typography variant="h6" gutterBottom>Conditions</Typography>
              <Typography variant="body2" color="text.secondary" gutterBottom>
                Add conditions to restrict when this route applies. All conditions must match.
              </Typography>
              
              {current.conditions && current.conditions.map((condition, index) => (
                <Accordion key={index} sx={{ mt: 1 }}>
                  <AccordionSummary expandIcon={<ExpandMoreIcon />}>
                    <Box display="flex" alignItems="center" justifyContent="space-between" width="100%">
                      <Typography>{condition.type} Condition</Typography>
                      <IconButton size="small" onClick={(e) => { e.stopPropagation(); removeCondition(index); }}>
                        <CloseIcon />
                      </IconButton>
                    </Box>
                  </AccordionSummary>
                  <AccordionDetails>
                    {condition.type === 'Time' && (
                      <Grid container spacing={2}>
                        <Grid item xs={6}>
                          <TextField fullWidth label="Start Time" type="time"
                            value={(condition as TimeCondition).start_time}
                            onChange={(e) => updateCondition(index, { ...condition, start_time: e.target.value } as TimeCondition)}
                            InputLabelProps={{ shrink: true }} />
                        </Grid>
                        <Grid item xs={6}>
                          <TextField fullWidth label="End Time" type="time"
                            value={(condition as TimeCondition).end_time}
                            onChange={(e) => updateCondition(index, { ...condition, end_time: e.target.value } as TimeCondition)}
                            InputLabelProps={{ shrink: true }} />
                        </Grid>
                      </Grid>
                    )}
                    
                    {condition.type === 'DayOfWeek' && (
                      <FormGroup>
                        <Typography variant="body2" gutterBottom>Select Days:</Typography>
                        {['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'].map((day, dayIndex) => (
                          <FormControlLabel key={day} control={
                            <Checkbox
                              checked={(condition as DayOfWeekCondition).days.includes(dayIndex + 1)}
                              onChange={(e) => {
                                const days = [...(condition as DayOfWeekCondition).days];
                                if (e.target.checked) {
                                  days.push(dayIndex + 1);
                                } else {
                                  const idx = days.indexOf(dayIndex + 1);
                                  if (idx > -1) days.splice(idx, 1);
                                }
                                updateCondition(index, { ...condition, days: days.sort() } as DayOfWeekCondition);
                              }}
                            />
                          } label={day} />
                        ))}
                      </FormGroup>
                    )}
                    
                    {condition.type === 'DateRange' && (
                      <Grid container spacing={2}>
                        <Grid item xs={6}>
                          <TextField fullWidth label="Start Date" type="date"
                            value={(condition as DateRangeCondition).start_date}
                            onChange={(e) => updateCondition(index, { ...condition, start_date: e.target.value } as DateRangeCondition)}
                            InputLabelProps={{ shrink: true }} />
                        </Grid>
                        <Grid item xs={6}>
                          <TextField fullWidth label="End Date" type="date"
                            value={(condition as DateRangeCondition).end_date}
                            onChange={(e) => updateCondition(index, { ...condition, end_date: e.target.value } as DateRangeCondition)}
                            InputLabelProps={{ shrink: true }} />
                        </Grid>
                      </Grid>
                    )}
                    
                    {condition.type === 'CallerId' && (
                      <Grid container spacing={2}>
                        <Grid item xs={10}>
                          <TextField fullWidth label="Caller ID Pattern (regex)"
                            value={(condition as CallerIdCondition).pattern}
                            onChange={(e) => updateCondition(index, { ...condition, pattern: e.target.value } as CallerIdCondition)}
                            placeholder="e.g., ^\+1\d{10}$ for US numbers" />
                        </Grid>
                        <Grid item xs={2}>
                          <FormControlLabel control={
                            <Checkbox
                              checked={(condition as CallerIdCondition).negate}
                              onChange={(e) => updateCondition(index, { ...condition, negate: e.target.checked } as CallerIdCondition)}
                            />
                          } label="Negate" />
                        </Grid>
                      </Grid>
                    )}
                    
                    {condition.type === 'Destination' && (
                      <Grid container spacing={2}>
                        <Grid item xs={10}>
                          <TextField fullWidth label="Destination Pattern (regex)"
                            value={(condition as DestinationCondition).pattern}
                            onChange={(e) => updateCondition(index, { ...condition, pattern: e.target.value } as DestinationCondition)}
                            placeholder="e.g., ^[2-5]\d{3}$ for extensions" />
                        </Grid>
                        <Grid item xs={2}>
                          <FormControlLabel control={
                            <Checkbox
                              checked={(condition as DestinationCondition).negate}
                              onChange={(e) => updateCondition(index, { ...condition, negate: e.target.checked } as DestinationCondition)}
                            />
                          } label="Negate" />
                        </Grid>
                      </Grid>
                    )}
                  </AccordionDetails>
                </Accordion>
              ))}

              <Stack direction="row" spacing={1} sx={{ mt: 2 }}>
                <Button size="small" onClick={() => addCondition('Time')}>+ Time</Button>
                <Button size="small" onClick={() => addCondition('DayOfWeek')}>+ Day of Week</Button>
                <Button size="small" onClick={() => addCondition('DateRange')}>+ Date Range</Button>
                <Button size="small" onClick={() => addCondition('CallerId')}>+ Caller ID</Button>
                <Button size="small" onClick={() => addCondition('Destination')}>+ Destination</Button>
              </Stack>
            </Grid>
          </Grid>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleSave} variant="contained" color="primary"
            disabled={!current.name || !current.pattern}>
            {editMode ? 'Update' : 'Create'}
          </Button>
        </DialogActions>
      </Dialog>

      {/* Test Dialog */}
      <Dialog open={testDialogOpen} onClose={() => setTestDialogOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Test Routing Rules</DialogTitle>
        <DialogContent>
          <Grid container spacing={2} sx={{ mt: 1 }}>
            <Grid item xs={12}>
              <TextField fullWidth label="Caller ID" value={testCallerId}
                onChange={(e) => setTestCallerId(e.target.value)}
                placeholder="+12125551234" />
            </Grid>
            <Grid item xs={12}>
              <TextField fullWidth label="Destination Number" value={testDestination}
                onChange={(e) => setTestDestination(e.target.value)}
                placeholder="2345" />
            </Grid>
            {testResult && (
              <Grid item xs={12}>
                <Alert severity={testResult.matched ? 'success' : 'info'}>
                  {testResult.matched ? (
                    <>
                      <Typography variant="body2"><strong>Matched Route:</strong> {testResult.route_name}</Typography>
                      <Typography variant="body2"><strong>Action:</strong> {testResult.action}</Typography>
                      <Typography variant="body2"><strong>Destination:</strong> {
                        typeof testResult.destination === 'object' 
                          ? `${testResult.destination.type}${testResult.destination.value ? `: ${testResult.destination.value}` : ''}`
                          : testResult.destination
                      }</Typography>
                    </>
                  ) : (
                    <Typography variant="body2">{testResult.message || 'No routes matched'}</Typography>
                  )}
                </Alert>
              </Grid>
            )}
          </Grid>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setTestDialogOpen(false)}>Close</Button>
          <Button onClick={handleTest} variant="contained" color="primary" startIcon={<TestIcon />}>
            Test
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
