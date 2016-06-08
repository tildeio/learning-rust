module AdventureGame
  class Location < Struct.new(:x, :y)
  end

  class Room < Struct.new(:location, :description)
    def self.new(x, y, description)
      super Location.new(x, y), description
    end

    def to_s
      "#{@location.x}, #{@location.y}, #{@description}"
    end
  end

  class Map < Struct.new(:title, :rooms)
    def initialize(title, room_list)
      rooms = []
      
      room_list.each do |room|
        x = room.location.x
        y = room.location.y

        rooms[x] ||= []
        rooms[x][y] = room
      end

      super title, rooms
    end

    def valid_directions(location)
      x = location.x
      y = location.y

      can_go_south = location.y > 0
      can_go_west  = location.x > 0

      can_go_east  = rooms[x + 1] && rooms[x + 1][y]
      can_go_north = rooms[x] && rooms[x][y + 1]   

      {
        north: !!can_go_north,
        south: !!can_go_south,
        east: !!can_go_east,
        west: !!can_go_west
      }
    end
  end
end