from pathlib import Path

import pytest
from finder import _URL
from finder import _available_appointments_exist
from httpx import Response

_TEST_FOLDER = Path(__file__).parent


@pytest.mark.parametrize(
    ("file_name", "expected_is_available"),
    [
        ("no_appointments.html", False),
        ("appointments.html", True),
    ],
)
def test_available_appointments_exist(
    respx_mock, file_name, expected_is_available
) -> None:
    content = _TEST_FOLDER.joinpath(file_name).read_text()
    respx_mock.get(_URL).mock(return_value=Response(200, content=content))
    is_available = _available_appointments_exist()
    assert is_available == expected_is_available
