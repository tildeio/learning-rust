describe AdventureGame::Map do
  def room(x, y, desc)
    AdventureGame::Room.new(x, y, desc)
  end

  context "small map" do
    let(:rooms) do
      [
        room(0, 2, "top left"),    room(1, 2, "top center"),    room(2, 2, "top right"),
        room(0, 1, "middle left"), room(1, 1, "middle center"), room(2, 1, "middle right"),
        room(0, 0, "bottom left"), room(1, 0, "bottom center"), room(2, 0, "bottom right")
      ]
    end

    subject(:map) do
      AdventureGame::Map.new("Liz's Great Adventure", rooms)
    end

    it "looks up rooms by coordinates" do
      expect(map.rooms[AdventureGame::Location.new(0, 2)]).to eq(rooms[0])
    end

    it "produces valid directions for top left" do
      location = AdventureGame::Location.new(0, 2)
      valid_directions = map.valid_directions(location)

      expect(valid_directions).to eq({
        north: false,
        south: true,
        east: true,
        west: false
      })
    end

    it "produces valid directions for bottom right" do
      location = AdventureGame::Location.new(2, 0)
      valid_directions = map.valid_directions(location)

      expect(valid_directions).to eq({
        north: true,
        south: false,
        east: false,
        west: true
      })
    end

    it "produces valid directions for the middle" do
      location = AdventureGame::Location.new(1, 1)
      valid_directions = map.valid_directions(location)

      expect(valid_directions).to eq({
        north: true,
        south: true,
        east: true,
        west: true
      })
    end
  end

  context "a bigger map" do
    def room(x, y)
      AdventureGame::Room.new(x, y, "#{x}, #{y}")
    end

    subject(:map) do
      rooms = [
        room(0, 3), room(1, 3), room(2, 3), room(3, 3),
        room(0, 2), room(1, 2), room(2, 2), room(3, 2),
        room(0, 1), room(1, 1), room(2, 1), room(3, 1),
        room(0, 0), room(1, 0), room(2, 0), room(3, 0),
      ]

      AdventureGame::Map.new("Liz's Great Adventure", rooms)
    end

    it "produces valid directions for top left" do
      location = AdventureGame::Location.new(0, 3)
      valid_directions = map.valid_directions(location)

      expect(valid_directions).to eq({
        north: false,
        south: true,
        east: true,
        west: false
      })
    end

    it "produces valid directions for 3, 2" do
      location = AdventureGame::Location.new(3, 2)
      valid_directions = map.valid_directions(location)

      expect(valid_directions).to eq({
        north: true,
        south: true,
        east: false,
        west: true
      })
    end
  end
end
