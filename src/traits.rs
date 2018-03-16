

use std::io::Result;

/// The WaveNode trait represents a single node which
/// exists within a given Wave Network. 
/// This means it supports Both Sending Sonar 
/// Pulses out to detect other WaveNodes in a given space, as well as listening to communication
/// signals sent from other potential nodes 
pub trait WaveNode {
    type Node; 

    /// Creates a new node on the network. 
    /// Note, this does not connect the node to the network,
    /// merely instantiates the object so that we can execute configuration prior to 
    /// adding it to the network
    fn new() -> Self::Node;

    /// Adds the node to the network.
    /// This means it engages the hardware responsible for outputting pulses and listening to
    /// pulses back.
    fn initialize(&self) -> Result<()>;
    
    /// Removes the node from the network, if it was engaged previously.
    /// Note, this also consumes the Node, so we must create a new one later.
    fn shutdown(self);

    
}
