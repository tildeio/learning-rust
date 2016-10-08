class Game
  def self.play
    Game.new.tap(&:play)
  end

  def initialize
    @valid_choices = [
      :help,
      :exit,
      :north,
      :south,
      :east,
      :west,
      :display_map,
      :look_around,
      :pick_up,
      :use,
      :print_inventory,
      :talk,
      :take
    ]

    @player = Player.new
    @playing = true
    @map = Map.new("Adventure Game", rooms, @player)
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


  # use with NPCs
  def take(item)
    if current_room.npc && current_room.npc.has_item(item)
      @player.add_to_inventory(item)
    else
      puts "Sorry, that item isn't here."
    end
  end

  # use for items in a Room
  def pick_up(item)
    if current_room.has_item(item)
      @player.add_to_inventory(item)
      current_room.remove_one(item)
    else
      puts "Sorry, that item is not in this room. Try again."
    end
  end

  def use(item)
    # TODO: figure out how to pass around entire item object to access effects anywhere
    if @player.has_item(item) && current_room.npc && current_room.npc.has_item(item)
      effect = current_room.npc.inventory.effects
      puts effect
      @player.remove_from_inventory(item)
      # TODO: eventually remove from NPC inventory & change ownership of item
    elsif @player.has_item(item) && current_room.has_item(item)
      effect = current_room.items.select { |i| i.name == item }.first.effects
      puts effect
      @player.remove_from_inventory(item)
      # TODO: eventually remove from Room inventory & change ownership of item
    else
      puts "Sorry, that item is not in your inventory. Did you pick it up or try taking it from someone?"
    end
  end

  def talk
    if current_room.npc
      puts current_room.npc.dialogue[:default]
    else
      puts "Sorry, no one else is in here!"
    end
  end

  def print_inventory
    @player.print_inventory
  end

  #TODO: clean this method up like whoa
  def parse_choice(choice)
    new_choice = choice.split.join("_")
    valid_choice = @valid_choices.select { |entry| entry == new_choice.to_sym }.first
    if [:north, :south, :east, :west].include?(valid_choice)
      move(valid_choice)
    elsif choice.include?('pick up')
      item = choice.split.select{ |item| item != 'pick' && item != 'up' }.join(' ')
      pick_up(item)
    elsif choice.include?('take')
      item = choice.split.select{ |item| item != 'take' }.join(' ')
      take(item)
    elsif choice.include?('use')
      item = choice.split.select{ |item| item != 'use' }.join(' ')
      use(item)
    elsif valid_choice != nil
      self.send valid_choice
    else
      check_validity(new_choice.to_sym)
    end
  end

  def display_map
    @map.display_map
  end

  def current_room
    rooms.select { |room| room[0] == @player.location }.first
  end

  def look_around
    puts current_room.description
    if current_room.has_items
      puts "This room contains #{current_room.item_list}"
    end
    puts current_room.npc.name + " is here too." if current_room.npc
  end

  def move(direction)
    if @map.valid_directions(@player.location)[direction]
      case direction
      when :north
        @player.location.y += 1
        puts "You have moved north! Your new location is #{@player.location.x}, #{@player.location.y}"
      when :south
        @player.location.y -= 1
        puts "You have moved south! Your new location is #{@player.location.x}, #{@player.location.y}"
      when :west
        @player.location.x -= 1
        puts "You have moved west! Your new location is #{@player.location.x}, #{@player.location.y}"
      when :east
        @player.location.x += 1
        puts "You have moved east! Your new location is #{@player.location.x}, #{@player.location.y}"
      else
        puts "Sorry, you can't move there!"
      end
    else
      puts "Sorry, you can't move there!"
    end
  end

  def help
    puts <<-HEREDOC
      exit: exit the game
      north, south, east, west: move in this direction
      look around: see a description of the current room
      pick up _item_: add the item to your inventory
      take _item_: take an item from an NPC
      use _item_: use an item in your inventory
      talk: talk to an NPC
      display map: look at map
      print inventory: show current player inventory
    HEREDOC
  end

  def exit
    @playing = false
    puts "Bye, #{@player.name}! Thanks for playing!"
  end

  def rooms
    [
      Room.new(
        0, 0,
        "Unicorn Room",
        "This room contains a rare and glorious unicorn. It's amazing.",
        [InventoryItem.new("a jar of unicorn farts", 1, nil, "BOOM. Unicorn farts are powerful! You are now very sparkly.")],
        NPC.new(
          "Unicorn Doctor",
          InventoryItem.new("vial of unicorn blood", 1, self, "The amazing unicorn blood has made you INVINCIBLE!"),
          {
            default: "Hi I'm a unicorn doctor. It's pretty cool. I have a vial of unicorn blood, do you want it?"
          }
        ),
      ),
      Room.new(
        0, 1,
        "Bear Room",
        "HOLY CRAP THERE'S A BEAR IN THIS ROOM.",
        [InventoryItem.new("a canister of bear repellant", 1, nil, "You are now safe from the bear! Stop stressin'")],
        nil
      ),
      Room.new(
        0, 2,
        "Cool Stuff Room",
        "This room's pretty cool, nbd",
        [nil],
        nil
      ),
      Room.new(
        1, 0,
        "Crappy Stuff Room",
        "Everything in this room stinks like garbage.",
        [InventoryItem.new("a garbage bomb", 3, nil, "Bad move, now you stink like garbage. But so does everything else.")],
        nil
      ),
      Room.new(
        1, 1,
        "Starting Out Room",
        "This is a room to start out in. Nothing to see here.",
        [nil],
        nil
      ),
      Room.new(
        1, 2,
        "Cute Puppy Room",
        "OMG this room is FULL. OF. PUPPIES. So many puppies!",
        [InventoryItem.new("a puppy", 10, nil, "You pet the heck out of the puppy. Look at his waggy tail! LOOK AT IT!! You are hypnotized.")],
        nil
      ),
      Room.new(
        2, 0,
        "Sandwich and Chips Room",
        "Yum, there's a sandwich and some chips in here!",
        [
          InventoryItem.new("a sandwich", 1, nil, "Yum, that was a good sandwich. It was made of whatever your favorite sandwich is."),
          InventoryItem.new("a bag of chips", 1, nil, "That was a delightful bag of chips! Crunchy as heck with perfect salt.")
        ],
        nil
      ),
      Room.new(
        2, 1,
        "Home Alone Room",
        "This room's got nothing in it. You're allll aloooone.",
        [nil],
        nil
      ),
      Room.new(
        2, 2,
        "Dank Meme Room",
        "This room is nothing but sweet memes.",
        [
          InventoryItem.new("a pic of Hillary Clinton texting", 1, nil, "You laugh yourself to sleep because memes are so funny right"),
          InventoryItem.new("a pic of a dog getting hit in the face with a frisbee", 1, nil, "Man that frisbee dog is hilarious, isn't he? Memes are the best.")
        ],
        nil
      )
    ]
  end

  private

  # TODO: refactor this like whoa, should work better with `parse_choice`
  # should check if Player input is valid
  def check_validity(choice)
    valid_options = ['pick up', 'use', 'display inventory']
    unless @valid_choices.include?(choice.to_sym) || valid_options.include?(choice)
      puts "That is not a valid choice. Try again."
    end
  end
end
