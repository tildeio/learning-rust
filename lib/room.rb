# A Room is a simple object that has a `location`,
# (which is a Location object) and a description
# (which is a string).
class Room < Struct.new(:location, :name, :description, :items, :npc)
  # `items` is the Room's inventory, which is empty
  # unless populated on Room initialization
  @items = []
  # The Room is constructed with an x and y, which
  # are passed to Location.new, so they should be
  # numbers.
  def self.new(x, y, name, description, items, npc)
    super Location.new(x, y), name, description, items, npc
  end

  # to_s prints out the x and y coordinates and
  # the name.
  def to_s
    "#{location.x}, #{location.y}, #{name}"
  end

  def has_items
    self.items.any?
  end

  # check to see if this Room has this item
  def has_item(item)
    item_names = []
    self.items.compact.each { |item| item_names << item.name }
    item_names.include? item
  end

  def remove_one(item)
    room_item = self.items.find { |i| i == item }
    # can't remove an item that's not there
    return if !room_item
    index = self.items.index(room_item)
    self.items.delete_at(index)
  end

  # puts-able list of items in Room
  def item_list
    if self.items.compact.count == 1
      self.items.compact.first.name
    else
      item_names = []
      self.items.compact.each { |item| item_names << item.name }
      item_names.join(", ")
    end
  end
end
