require 'spec_helper'

describe Location do
  let(:subject) { described_class.new(1,2) }

  it 'has an x' do
    expect(subject.x).to eq 1
  end

  it 'has a y' do
    expect(subject.y).to eq 2
  end
end
