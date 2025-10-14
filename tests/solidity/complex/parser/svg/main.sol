// SPDX-License-Identifier: MIT

pragma solidity >=0.8.4;

import "./base64.sol";
import "./Strings.sol";
import "./PRBMathUD60x18.sol";

/**
 * @title Land SVG Library
 *
 * @notice Provide functions to generate SVG image representation of the LandERC721, and other
 *      auxiliary functions to construct token metadata JSON, and encode it into base64 format.
 *
 * @dev The metadata format, returned by `constructTokenURI` function conforms with the official
 *    ERC721 metadata standard.
 *
 * @dev There are basically 3 components of the metadata schema, the name, description and the image itself.
 *    each of them have it's generating functions, `generateLandName`, `generateLandDescription` and `_generateSVGImage`.
 *
 * @dev The output of `_generateSVGImage` will be encoded as Base64 so that the browser can interpret it, as well as the
 *    entire output of `constructTokenURI`.
 *
 * @dev LandDescriptor should call `constructTokenURI` in order to get the encoded ERC721 metadata.
 *
 * @author Pedro Bergamini, Yuri Fernandes, Estevan Wisoczynski
 */
contract LandSvgLib {
    using Strings for uint256;
    using PRBMathUD60x18 for uint256;

    struct Site {
        /**
         * @dev Site type:
         *        1) Carbon (element),
         *        2) Silicon (element),
         *        3) Hydrogen (element),
         *        4) Crypton (fuel),
         *        5) Hyperion (fuel),
         *        6) Solon (fuel)
         */
        uint8 typeId;

        /**
         * @dev x-coordinate within a plot
    	 */
        uint16 x;

        /**
         * @dev y-coordinate within a plot
		 */
        uint16 y;
    }

    /**
     * @dev Generate the Land SVG image.
	 *
	 * @param _gridSize The size of the grid
	 * @param _tierId PlotView.tierId land tier id
	 * @param _landmarkTypeId landmark type id
	 * @param _sites array of LandLib.Site
	 * @return Land SVG image
	 */
    function _generateSVGImage(
        uint16 _gridSize,
        uint8 _tierId,
        uint8 _landmarkTypeId,
        Site[] memory _sites
    ) public pure returns (string memory) {
        // Multiply by 3 to get number of grid squares = dimension of the isomorphic grid size

        return string(
            abi.encodePacked(
                "<svg height='",
                uint256(_gridSize * 3 + 6).toString(),
                "' width='",
                uint256(_gridSize * 3).toString(),
                "' stroke-width='2' xmlns='http://www.w3.org/2000/svg'>",
                "<rect rx='5%' ry='5%' width='100%' height='99%' fill='url(#BOARD_BOTTOM_BORDER_COLOR_TIER_",
                uint256(_tierId).toString(),
                ")' stroke='none'/>",
                "<svg height='97.6%' width='100%' stroke-width='2' xmlns='http://www.w3.org/2000/svg'>",
                _generateLandBoard(_gridSize, _tierId, _landmarkTypeId, _sites), // This line should be replaced in the loop
                "</svg>"
            )
        );
    }

    /**
     * @dev Returns the site base svg array component, used to represent
	 *      a site inside the land board.
	 *
	 * @param _x Sites.x position
	 * @param _y Sites.y position
	 * @param _typeId Sites.typeId
	 * @return The base SVG element for the sites
	 */
    function _siteBaseSvg(uint16 _x, uint16 _y, uint8 _typeId) public pure returns (string memory) {
        return string(
            abi.encodePacked(
                "<svg x='",
                uint256(_x).toString(),
                "' y='",
                uint256(_y).toString(),
                "' width='6' height='6' xmlns='http://www.w3.org/2000/svg'><use href='#SITE_TYPE_",
                uint256(_typeId).toString(),
                "' /></svg>"
            )
        );
    }

    /**
     * @dev Returns the site base svg array component, used to represent
	 *      a landmark inside the land board.
	 *
	 * @param _gridSize The size of the grid
	 * @param _landmarkTypeId landmark type defined by its ID
	 * @return Concatenation of the landmark SVG component to be added the board SVG
	 */
    function _generateLandmarkSvg(uint16 _gridSize, uint8 _landmarkTypeId) public pure returns (string memory) {
        uint256 landmarkPos = uint256(_gridSize - 2).fromUint().div(uint256(2).fromUint()).mul(uint256(3).fromUint());

        string memory landmarkFloatX;
        string memory landmarkFloatY;
        if (_gridSize % 2 == 0) {
            landmarkFloatX = landmarkPos.toUint().toString();
            landmarkFloatY = (landmarkPos.toUint() - 3).toString();
        } else {
            landmarkFloatX = (landmarkPos.ceil().toUint() + 1).toString();
            landmarkFloatY = (landmarkPos.floor().toUint() - 1).toString();
        }

        return string(
            abi.encodePacked(
                "<svg x='",
                landmarkFloatX,
                "' y='",
                landmarkFloatY,
                "' width='12' height='12' xmlns='http://www.w3.org/2000/svg'><use href='#LANDMARK_TYPE_",
                uint256(_landmarkTypeId).toString(),
                "'/></svg>"
            )
        );
    }

    /**
     * @dev Returns the land board base svg array component, which has its color changed
	 *      later in other functions.
	 *
	 * @param _gridSize The size of the grid
	 * @param _tierId PlotView.tierId land tier id
	 * @param _landmarkTypeId landmark type id
	 * @param _sites array of LandLib.Site
	 * @return Array of board SVG component parts
	 */
    function _landBoardArray(
        uint16 _gridSize,
        uint8 _tierId,
        uint8 _landmarkTypeId,
        Site[] memory _sites
    ) public pure returns (string[11] memory) {
        uint256 scaledGridSize = uint256(_gridSize).fromUint().div(uint256(2).fromUint()).mul(uint256(3).fromUint());
        string memory scaledGridSizeString = string(
            abi.encodePacked(
                scaledGridSize.toUint().toString(),
                ".",
                truncateString(scaledGridSize.frac().toString(), 0, 2)
            )
        );
        return [
            "<defs><symbol id='SITE_TYPE_1' width='6' height='6'>", // Site Carbon
            string(abi.encodePacked(scaledGridSizeString, " ", scaledGridSizeString)),
            "</defs><rect width='100%' height='100%' fill='url(#GRADIENT_BOARD_TIER_",
            uint256(_tierId).toString(), // This line should be replaced in the loop
            ")' stroke='none' rx='5%' ry='5%'/><svg x='",
            scaledGridSizeString,
            ")'>",
            _generateLandmarkSvg(_gridSize, _landmarkTypeId), // Generate LandMark SVG
            _generateSites(_sites), // Generate Sites SVG
            uint256(_tierId).toString(),
            ")' stroke-width='1' rx='4.5%' ry='4.5%'/></svg>"
        ];
    }

    /**
    * @dev Return the concatenated Land Board SVG string
	*
	* @param _gridSize The size of the grid
	* @param _tierId PlotView.tierId land tier id
	* @param _landmarkTypeId landmark type id
	* @param _sites array of LandLib.Site
	* @return Land Board SVG string
	*/
    function _generateLandBoard(
        uint16 _gridSize,
        uint8 _tierId,
        uint8 _landmarkTypeId,
        Site[] memory _sites
    ) public pure returns(string memory) {
        string[11] memory landBoardArray_ = _landBoardArray(
            _gridSize,
            _tierId,
            _landmarkTypeId,
            _sites
        );
        bytes memory landBoardBytes;
        for (uint8 i = 0; i < landBoardArray_.length; i++) {
            landBoardBytes = abi.encodePacked(landBoardBytes, landBoardArray_[i]);
        }

        return string(landBoardBytes);
    }

    /**
     * @dev Calculates string for the land name based on plot data.
	 *
	 * @param _regionId PlotView.regionId
	 * @param _x PlotView.x coordinate
	 * @param _y PlotView.y coordinate
	 * @return SVG name attribute
	 */
    function generateLandName(uint8 _regionId, uint16 _x, uint16 _y) public pure returns (string memory) {
        string memory region;
        if (_regionId == 1) {
            region = "Taiga Boreal";
        } else if (_regionId == 2) {
            region = "Crystal Shores";
        } else if (_regionId == 3) {
            region = "Shardbluff Labyrinth";
        } else if (_regionId == 4) {
            region = "Abyssal Basin";
        } else if (_regionId == 5) {
            region = "Crimson Waste";
        } else if (_regionId == 6) {
            region = "Brightland Steppes";
        } else if (_regionId == 7) {
            region = "Halcyon Sea";
        } else {
            revert("Invalid region ID");
        }
        return string(
            abi.encodePacked(
                region,
                " (",
                uint256(_x).toString(),
                ", ",
                uint256(_y).toString(),
                ")"
            )
        );
    }

    /**
     * @dev Returns the string for the land metadata description.
	 */
    function generateLandDescription() public pure returns (string memory) {
        return "Illuvium Land is a digital piece of real estate in the Illuvium universe that players can mine for fuels through Illuvium Zero. "
        "Fuels are ERC-20 tokens that are used in Illuvium games and can be traded on the marketplace. Higher-tiered lands produce more fuel."
        "\\n\\nLearn more about Illuvium Land at illuvidex.illuvium.io/land.";
    }

    /**
     * @dev Generates each site inside the land svg board with is position and color.
	 *
	 * @param _sites Array of plot sites coming from PlotView struct
	 * @return The sites components for the land SVG
	 */
    function _generateSites(Site[] memory _sites) public pure returns (string memory) {
        bytes memory _siteSvgBytes;
        for (uint256 i = 0; i < _sites.length; i++) {
            _siteSvgBytes = abi.encodePacked(
                _siteSvgBytes,
                _siteBaseSvg(
                    convertToSvgPositionX(_sites[i].x),
                    convertToSvgPositionY(_sites[i].y),
                    _sites[i].typeId
                )
            );
        }

        return string(_siteSvgBytes);
    }

    /**
     * @dev Main function, entry point to generate the complete land svg with all
	 *      populated sites, correct color, and attach to the JSON metadata file
	 *      created using Base64 lib.
	 * @dev Returns the JSON metadata formatted file used by NFT platforms to display
	 *      the land data.
	 * @dev Can be updated in the future to change the way land name, description, image
	 *      and other traits are displayed.
	 *
	 * @param _regionId PlotView.regionId
	 * @param _x PlotView.x coordinate
	 * @param _y PlotView.y coordinate
	 * @param _tierId PlotView.tierId land tier id
	 * @param _gridSize The size of the grid
	 * @param _landmarkTypeId landmark type defined by its ID
	 * @param _sites Array of plot sites coming from PlotView struct
	 */
    function constructTokenURI(
        uint8 _regionId,
        uint16 _x,
        uint16 _y,
        uint8 _tierId,
        uint16 _gridSize,
        uint8 _landmarkTypeId,
        Site[] memory _sites
    ) public pure returns (string memory) {
        string memory name = generateLandName(_regionId, _x, _y);
        string memory description = generateLandDescription();
        string memory image = Base64.encode(
            bytes(
                _generateSVGImage(
                    _gridSize,
                    _tierId,
                    _landmarkTypeId,
                    _sites
                )
            )
        );

        return string(
            abi.encodePacked("data:application/json;base64, ", Base64.encode(
                bytes(
                    abi.encodePacked('{"name":"',
                    name,
                    '", "description":"',
                    description,
                    '", "image": "',
                    'data:image/svg+xml;base64,',
                    image,
                    '"}')
                )
            )
            )
        );
    }

    /**
     * @dev Convert site X position to fit into the board.
	 *
	 * @param _positionX X coordinate of the site
	 * @return Transformed X coordinate
	 */
    function convertToSvgPositionX(uint16 _positionX) public pure returns (uint16) {
        return _positionX * 3;
    }

    /**
     * @dev Convert site Y position to fit into the board.
	 *
	 * @param _positionY Y coordinate of the site
	 * @return Transformed Y coordinate
	 */
    function convertToSvgPositionY(uint16 _positionY) public pure returns (uint16) {
        return _positionY * 3;
    }

    /**
     * @dev Truncate string at a certain position and size.
	 *
	 * @param _str String to be truncated
	 * @param _from The initial position to start slicing
	 * @param _size The size of the resulting substring
	 * @return Truncated string
	 */
    function truncateString(string memory _str, uint256 _from, uint256 _size) public pure returns (string memory) {
        bytes memory stringBytes = bytes(_str);
        if (_from + _size >= stringBytes.length) {
            return _str;
        }

        bytes memory truncatedBytes = new bytes(_size);
        uint256 j;
        for (uint256 i = _from; i < _from + _size; i++) {
            truncatedBytes[j] = stringBytes[i];
            j++;
        }

        return string(truncatedBytes);
    }
}