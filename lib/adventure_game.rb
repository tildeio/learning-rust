module AdventureGame
require 'pry'
  # A Location is a simple object that has an `x` and
  # `y` coordinate. Both are numbers.
  class Location < Struct.new(:x, :y)
  end

  # A Room is a simple object that has a `location`,
  # (which is a Location object) and a description
  # (which is a string).
  class Room < Struct.new(:location, :description)
    # The Room is constructed with an x and y, which
    # are passed to Location.new, so they should be
    # numbers.
    def self.new(x, y, description)
      super Location.new(x, y), description
    end

    # to_s prints out the x and y coordinates and
    # the description.
    def to_s
      "#{location.x}, #{location.y}, #{description}"
    end
  end

  class Player < Struct.new(:name, :inventory)
    attr_accessor :name

    def initialize
      puts "What is your name?"
      @name = gets.chomp
      @inventory = []
    end

    def print_inventory
      if @inventory.length > 0
        @inventory.each_with_index do |item, index|
          puts "#{index + 1}. item"
        end
      end
    end
  end

  class Game

    def initialize
      @valid_choices = [
        :help,
        :exit,
        :north,
        :south,
        :east,
        :west,
        :display_map,
        :look_around
      ]

      @player = Player.new
      @playing = true
      @map = Map.new("Adventure Game", rooms)
      play
    end

    def play
      @map.display_map
      puts "Hi, #{@player.name}. What would you like to do?"
      parse_choice(gets.chomp)
      while @playing
        puts "What now?"
        choice = gets.chomp
        parse_choice(choice)
        break if !@playing
      end
    end


    def parse_choice(choice)
      choice = choice.split.join("_").to_sym
      check_validity(choice)
      valid_choice = @valid_choices.select { |entry| entry == choice.to_sym }.first
      self.send valid_choice if valid_choice != nil
    end

    def display_map
      @map.display_map
    end

    def help
      puts <<-HEREDOC
        exit: exit the game
        north, south, east, west: move in this direction
        look around: see a description of the current room
        pick up _item_: add the item to your inventory
        use _item_: use an item in your inventory
        display map: look at map
      HEREDOC
    end

    def exit
      @playing = false
      puts "Bye, #{@player.name}! Thanks for playing!"
    end

    def rooms
      [
        Room.new(0, 0, "Unicorn Room"),
        Room.new(0, 1, "Bear Room"),
        Room.new(0, 2, "Cool Stuff Room"),
        Room.new(1, 0, "Crappy Stuff Room"),
        Room.new(1, 1, "Starting Out Room"),
        Room.new(1, 2, "Cute Puppy Room"),
        Room.new(2, 0, "Sandwich and Chips Room"),
        Room.new(2, 1, "Home Alone Room"),
        Room.new(2, 2, "Dank Meme Room")
      ]
    end

    private

    def check_validity(choice)
      unless @valid_choices.include? choice
        puts "That is not a valid choice. Try again."
      end
    end
  end

  # A Map is an object that has a `title` as a string,
  # a Hash of rooms (where the keys are Locations),
  # a max_x, which is the east-most room, and max_y,
  # which is the north-most room.
  class Map < Struct.new(:title, :rooms, :max_x, :max_y)
    # A Map is initialized with a title and room list,
    # which is an array of rooms, and the room list is
    # converted into the expected Hash and max_x/max_y
    # values.
    def initialize(title, room_list)
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
        puts "#{index}. #{room[1].description}"
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
end

AdventureGame::Game.new
