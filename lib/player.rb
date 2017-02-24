class Player
  attr_accessor :name, :inventory, :location

  def initialize
    @location = Location.new(0,0)
    puts "What is your name?"
    @name = gets.chomp
    @inventory = []
  end

  def has_item(item_name)
    self.inventory.map(&:name).include?(item_name)
  end

  def add_to_inventory(item)
    self.inventory.push(item)
    puts "#{item.name.capitalize} has been added to your inventory."
  end

  def remove_from_inventory(item_name)
    if @inventory.delete_if { |i| i.name == item_name }
      puts "#{item_name} has been removed from your inventory"
    else
      puts "Sorry, you don't have #{item_name}"
    end
  end

  def print_inventory
    if @inventory.length > 0
      puts "\n"
      puts "INVENTORY"
      puts "========="
      @inventory.each do |item|
        puts "* #{item.name}: #{item.count}"
      end
      puts "\n"
    else
      puts "Sorry, your inventory is empty!"
      puts "Why not look around and see if you can find something to pick up!"
    end
  end
end
