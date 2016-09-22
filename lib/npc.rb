# NPC = Non Player Character
class NPC
  attr_reader :name, :inventory, :dialogue

  def initialize(name, inventory, dialogue)
    @name = name
    @inventory = inventory
    @dialogue = dialogue
  end

  def has_item(item)
    self.inventory.name == item
  end
end