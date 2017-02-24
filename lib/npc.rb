require 'pry'
# NPC = Non Player Character
class NPC
  attr_reader :name, :dialogue
  attr_accessor :inventory

  def initialize(name, inventory, dialogue)
    @name = name
    @inventory = inventory
    @dialogue = dialogue
  end

  def has_item(item)
    @inventory.map(&:name).include?(item)
  end

  def remove_from_inventory(item)
    @inventory.delete_if { |i| item.name == i.name }
    puts "#{item.name} has been removed from #{self.name}'s inventory."
  end
end
