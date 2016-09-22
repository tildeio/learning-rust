class InventoryItem
  attr_accessor :count, :owner
  attr_reader :name, :effects

  def initialize(name, count, owner, effects)
      @name = name
      @count = count
      @owner = owner # owner can only be Player or Room
      @effects = effects
  end
end