pragma solidity ^0.6.0;

contract ERC20 {

    mapping (address => uint256) private _balances;
    mapping (address => mapping (address => uint256)) private _allowances;
    uint256 private _totalSupply;
    string private _name;
    string private _symbol;
    uint8 private _decimals;

    // phantom
    address msg_sender;

    constructor (string tokenName, string tokenSymbol2, uint256 tokenTotalSupply) public {
        _name = tokenName;
        _symbol = tokenSymbol2;
        _decimals = 18;
        _balances[msg_sender] = tokenTotalSupply;
        _totalSupply = tokenTotalSupply;
    }

    function name() public view returns (string) {
        return _name;
    }

    function symbol() public view returns (string) {
        return _symbol;
    }

    function decimals() public view returns (uint8) {
        return _decimals;
    }

    function totalSupply() public view returns (uint256) {
        return _totalSupply;
    }

    function balanceOf(address account) public view returns (uint256) {
        return _balances[account];
    }

    function transfer(address recipient, uint256 amount) public {
        _transfer(msg_sender, recipient, amount);
    }

    function allowance(address owner, address spender) public returns (uint256) {
        return _allowances[owner][spender];
    }

    function approve(address spender, uint256 amount) public {
        _approve(msg_sender, spender, amount);
    }

    function transferFrom(address owner, address recipient, uint256 amount) public {
        _transfer(owner, recipient, amount);
        _approve(owner, msg_sender, _allowances[owner][msg_sender] - amount);
    }

    function _transfer(address sender, address recipient, uint256 amount) internal {
        _balances[sender] = _balances[sender] - amount;
        _balances[recipient] = _balances[recipient] + amount;
    }

    function _approve(address owner, address spender, uint256 amount) internal {
        _allowances[owner][spender] = amount;
    }
}
