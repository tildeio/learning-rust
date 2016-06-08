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

  class Map < Struct.new(:title, :rooms, :max_x, :max_y)
    def initialize(title, room_list)
      rooms = {}
      max_x = 0
      max_y = 0

      room_list.each do |room|
        x = room.location.x
        y = room.location.y

        max_x = [max_x, x].max
        max_y = [max_y, y].max

        rooms[room.location] = room
      end

      super title, rooms, max_x, max_y
    end

    def [](location)
      self.rooms[location]
    end

    def valid_directions(location)
      {
        north: location.y < self.max_y,
        south: location.y > 0,
        east: location.x < self.max_x,
        west: location.x > 0
      }
    end
  end
end