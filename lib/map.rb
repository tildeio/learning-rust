# A Map is an object that has a `title` as a string,
# a Hash of rooms (where the keys are Locations),
# a max_x, which is the east-most room, and max_y,
# which is the north-most room.
class Map < Struct.new(:title, :rooms, :max_x, :max_y)
  # A Map is initialized with a title and room list,
  # which is an array of rooms, and the room list is
  # converted into the expected Hash and max_x/max_y
  # values.
  def initialize(title, room_list, player)
    @player = player
    # Construct an empty Hash and initialize max_x and max_y
    rooms = {}
    max_x = 0
    max_y = 0

    # Iterate over the room_list
    room_list.each do |room|
      # extract x and y from the room's location
      x = room.location.x
      y = room.location.y

      #update max_x and max_y if necessary
      max_x = [max_x, x].max
      max_y = [max_y, y].max

      #insert the room into the rooms Hash with its location
      rooms[room.location] = room
    end

    # construct a new Map with title, rooms, max_x and max_y
    super title, rooms, max_x, max_y
  end

  def display_map
    puts title
    title.length.times do
      print "="
    end
    puts "\n"
    rooms.each_with_index do |room, index|
      if room[0] == @player.location
        puts "#{index}. #{room[1].name}. You are here."
      else
        puts "#{index}. #{room[1].name}"
      end
    end
  end

  def [](location)
    self.rooms[location]
  end

  # valid_directions takes a Location contained in this
  # Map and answers which directions a player can go.
  # Specifically, a player is not allowed to move off
  # the edge of the map.
  def valid_directions(location)
    {
      # a player can go north if they are not already
      # at the max_y
      north: location.y < self.max_y,
      # a player can go south if they are not at the
      # bottom edge
      south: location.y > 0,
      # a player can go east if they are not already
      # at the max_x
      east: location.x < self.max_x,
      # a player can go west if they are not at the
      # left edge
      west: location.x > 0
    }
  end
end