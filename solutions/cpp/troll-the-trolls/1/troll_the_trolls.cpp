namespace hellmath {

// TODO: Task 1 - Define an `AccountStatus` enumeration to represent the four
// account types: `troll`, `guest`, `user`, and `mod`.

enum class AccountStatus { troll, guest, user, mod };
// TODO: Task 1 - Define an `Action` enumeration to represent the three
// permission types: `read`, `write`, and `remove`.
enum class Action { read, write, remove };

// TODO: Task 2 - Implement the `display_post` function, that gets two arguments
// of `AccountStatus` and returns a `bool`. The first argument is the status of
// the poster, the second one is the status of the viewer.

bool display_post(AccountStatus poster, AccountStatus viewer) {
  switch (poster) {
  case AccountStatus::troll:
    if (viewer == AccountStatus::troll) {
      return true;
    } else {
      return false;
    }
  default:
    return true;
  }
}

// TODO: Task 3 - Implement the `permission_check` function, that takes an
// `Action` as a first argument and an `AccountStatus` to check against. It
// should return a `bool`.

bool permission_check(Action permission, AccountStatus status) {
    switch (permission) {
      case Action::read:
         return true;
      case Action::write:
         switch (status) {
            case AccountStatus::guest:
               return false;
            default:
               return true;
         }
      case Action::remove:
         switch (status) {
            case AccountStatus::mod:
               return true;
            default:
               return false;
         }
    }
    return false;
}

// TODO: Task 4 - Implement the `valid_player_combination` function that
// checks if two players can join the same game. The function has two parameters
// of type `AccountStatus` and returns a `bool`.
bool valid_player_combination(AccountStatus p1, AccountStatus p2) {
  if (p1 == AccountStatus::guest || p2 == AccountStatus::guest ) {
    return false;
  }

  if (p1 == AccountStatus::troll || p2 == AccountStatus::troll ) {

    if (p1 == AccountStatus::troll && p2 == AccountStatus::troll ) {
      return true;
    } else {
      return false;
    }
  }
  return true;
}


// TODO: Task 5 - Implement the `has_priority` function that takes two
// `AccountStatus` arguments and returns `true`, if and only if the first
// account has a strictly higher priority than the second.

bool has_priority(AccountStatus p1, AccountStatus p2) {
  if( p1 > p2 ) {
    return true;
  } 
  return false;
}

} // namespace hellmath
