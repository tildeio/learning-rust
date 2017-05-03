require 'spec_helper'

describe InventoryItem do
  let(:subject) { described_class.new(name,count,owner,effects) }

  let(:name) { 'Candlestick' }
  let(:count) { 2 }
  let(:owner) { double('Player') }
  let(:effects) { 'Lighting the candlestick illuminates the room and possibly a dangerous weapon in the right hands.' }


  it 'has a name' do
    expect(subject.name).to eq name
  end

  it 'has a count' do
    expect(subject.count).to eq count
  end

  it 'can have the count increased' do
    subject.count += 1
    expect(subject.count).to eq(count + 1)
  end

  it 'can have the count decreased' do
    subject.count -= 1
    expect(subject.count).to eq(count - 1)
  end

  it 'has an owner' do
    expect(subject.owner).to eq owner
  end

  it 'has effects' do
    expect(subject.effects).to eq effects
  end
end
