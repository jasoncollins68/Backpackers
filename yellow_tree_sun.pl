#!/usr/bin/perl

# This program is used to allow backpackers to search and book accommodations 
# from different countries. 

use strict;
use warnings;
use IO::Socket;

# Create a list of countries
my @countries = ("Australia", "New Zealand", "Bhutan", "Thailand", "Vietnam",
 "South Africa", "Kenya", "Ecuador", "Iceland", "Indonesia", " Chile", 
" Nepal", "Cambodia", "Myanmar", "Costa Rica", "Namibia");

# Create a hash mapping countries with their respective locations
my %country_coordinates = (
	"Australia" => "25.274398,133.775136",
	"New Zealand" => "-40.900557,174.885971",
	"Bhutan" => "27.514162,90.433601",
	"Thailand" => "15.870032,100.992541",
	"Vietnam" => "14.058324,108.277199",
	"South Africa" => "-30.559482,22.937506",
	"Kenya" => "-0.023559,37.906193",
	"Ecuador" => "-1.831239,-78.183406",
	"Iceland" => "64.963051,-19.020835",
	"Indonesia" => "-0.789275,113.921327",
	"Chile" => "-35.675147,-71.542969",
	"Nepal" => "28.394857,84.124008",
	"Cambodia" => "12.565679,104.990963",
	"Myanmar" => "21.916221,95.955974",
	"Costa Rica" => "9.748917,-83.753428",
	"Namibia" => "-22.957640,18.490410"
);

# Ask the user to enter a country
print "Enter the country you wish to travel to: ";
my $country = <STDIN>;
chomp($country);

# Check if the given country exists in our list
if (!grep(/^$country$/, @countries)) {
	print "We don't offer services in the given country.\n";
	exit;
}

# Create the socket connection 
my $socket = new IO::Socket::INET (
	PeerHost => 'www.backpackers.com',
	PeerPort => '80',
	Proto => 'tcp',
);

# Die if connection is not successful
die "cannot connect to the server $!\n" unless $socket;

# Construct the request
my $request = "GET /accommodations/?country=$country";
$request .= "&location=$country_coordinates{$country} HTTP/1.0\r\n\r\n";

# Print the request
print $socket $request;

# Read the response
my $response = "";
while (my $line = <$socket>) {
	$response .= $line;
}

# Close the socket
close($socket);

# Parse the response to get the list of accommodations
my ($accommodations) = $response =~ /<h3>(.*)<\/h3>/;

# Print all the available accommodations
my @accommodations_list = split(/\s*<br \/>\s*/, $accommodations);
print "The list of accommodations:\n";
foreach my $accommodation (@accommodations_list) {
	print "$accommodation\n";
}