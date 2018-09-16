# hex-grid-toolbox-rs

Note: This repo is still being heavily restructured.

## Overview

Library providing tools for working with hexagonal grids and coordinate systems in rust. 
Heavily inspired and informed by the brilliant [Red Blob Games Hexagonal Grid Reference](https://www.redblobgames.com/grids/hexagons/) .

## Working with hexagonal coordinate systems

The primary goal of this library is to provide a simple interface to abstract away the complexities of working with hexagonal grids and coordinate systems. For a great overview of the different hexagonal coordinate systems we support and why they're good/bad look [here](https://www.redblobgames.com/grids/hexagons/#coordinates).

### What is a coordinate

In the context of this library a coordinate is a set of integers which define, for a specific coordinate system, the position of a hexagon within a grid, combined with an enum (called `Orientation`) which specifies the orientation of the hex.

### What is a pixel coordinate

One significant behaviour shared by all coordinate systems is that they can all be translated into positions in a 2d pixel grid, which is necessary for display. We provide the `PixelCoord` type to facilitate this. Translating a coordinate into a pixel coordinate requires a radius for the hex be specified, and creating a new pixel coordinate to be transformed into a hex coordinate requires a radius and orientation be specified using the `HexShape` enum.

### Which coordinate system should I use

One of the main goals of this library is abstracting away the implimentation details of the different coordinate systems to a point that it doesn't make much of a difference which coordinate system you're using when writing code. It should be relatively painless to start working with offset coordinates and later move to using cube coordinates later in the project.

Having said that, most of the algorithms for hexagonal coordinates are more efficient using cube or axial coordinates, and in fact many of the implimentations in this library will convert coordinates to cube or axial form and back to solve specific problems because that is by far the most efficient way to do it. The performance cost of those conversions should be low, but if performance is required then storing coordinates in cube or axial form (converting between cube and axial is trivial so both forms can be considered equivalent) may be the best approach.

Also consider that it is possible to easily convert a coordinate in any system into the corresponding coordinate in any other system with the rust `from/into` syntax, so you can use different coordinate systems in different parts of your program and still combine those representations when necessary. 

## HexTile and HexGrid

The library also provides some generic types to support operating on a grid of hexagonal tiles.

The `HexTile` type provides a lightweight generic way to store a coordinate alongside some arbetrary data.

The `HexGrid` type provides a way to store and oeprate on a collection of HexTiles. The library currently only provides a method for storing tiles as elements in a dictionary, but other storage methods can be added using the `HexGridDataWrapper` trait. 

The `HexGrid` type also provides a mechanism for constructing the grid by passing in an iterator which returns coordinate values, and a set of iterators that do that for basic map shapes, but that interface is currently too weak and needs to be revised.

## Example UI

The project includes an example of the library being used to render a hexagonal grid to the screen. 
The example can be found at `/examples/ui/` and can be run with the command: 

```text
cargo run --example ui
```

This example is still improving with the library, and should eventually showcase the various algorithms working for various coordinate systems.

## Requirements

This library currently has no requirements.

The UI example requires the ggez game library.

## TODO

- Improve example binary UI to allow switching between coordinate systems, orientation, and map modes at runtime to easily review behaviours.
- Add the missing common algorithms and add to example binary.
- Add tests


## To Consider

- Adding directionality as a concept (aka I'm facing north on a flat-top hex, what's in front of me, what can I see, etc)
- Reducing current limitations on types used by the coordinate systems by allow generic numeric types if/where feasable.
- Adding support for basic arithmetic expressions on coordinate types (may not make total sense for offset coordinate types?)
- Reconsidering whether it's practical to impliment a generic array storage over all coordinate types. 