class Player
  attr_accessor :name, :inventory, :location

  def initialize
    @location = Location.new(0,0)
    puts "What is your name?"
    @name = gets.chomp
    @inventory = {}
  end

  def has_item(item)
    @inventory.include?(item)
  end

  def add_to_inventory(item)
    has_item(item) ? @inventory[item] += 1 : @inventory[item] = 1
    puts "#{item.capitalize} has been added to your inventory."
  end

  def remove_from_inventory(item)
    if @inventory[item] && @inventory[item] > 0
      @inventory[item] -= 1
    else
      puts "Sorry, you don't have #{item}"
    end
  end

  def print_inventory
    if @inventory.keys.length > 0
      puts "\n"
      puts "INVENTORY"
      puts "========="
      @inventory.each_pair do |item, count|
        puts "* #{item}: #{count}"
      end
      puts "\n"
    else
      puts "Sorry, your inventory is empty!"
      puts "Why not look around and see if you can find something to pick up!"
    end
  end
end
