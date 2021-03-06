# AoC 2019 #20

Perhaps rather than working entirely in the maze map coordinates, we could
transform it into a list of shortest-distances between portals, then look
for the shortest path through them.

It looks like all the portals occur on either the inner or outer edges
of the donut, not at arbitrary places inside. 

*When on an empty square next to a portal, a single step takes you to the 
other tile with the same label.*

So we want to parse the input into, as a first pass, a map of wall/space
binaries, and a list of 2-letter labels of portals onto squares.

Actually, perhaps it's easier to read it into a 2d matrix of chars,
then parse from that - will make it a little easier to deal with finding
letters above/below each other.

...

Did one initial step of finding the labels and the squares that they mark.

It seems like it'd never be interesting to walk anywhere other than the
shortest path between two portals... On the other hand a two-stage search
seems harder to implement.

One way to describe this would be as a list of squares that are one step from
each other: either vacant `.` squares, or portal edges. Then we could do a
shortest-path-first search through these.

... 

Possibly, parsing the passages out of the map into an adjacency matrix 
is unnecessary: we could pretty easily have looked them up in the character
map on demand. But it makes the code a little easier and perhaps will help in
part B.
